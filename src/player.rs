use bevy::prelude::*;

use crate::mob::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

pub fn update_velocity(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Acceleration), With<Player>>) {
	let (mut velocity, acceleration) = query.single_mut();
	let velocity_vec = &mut velocity.0;

	let mut acceleration_vec = Vec2::ZERO;
	let mut passive_deceleration = Vec2::ZERO;

	if keyboard_input.pressed(KeyCode::A) {
		acceleration_vec += Vec2::NEG_X;
		velocity_vec.x = f32::min(0.0, velocity_vec.x);
	} else if keyboard_input.pressed(KeyCode::D) {
		acceleration_vec += Vec2::X;
		velocity_vec.x = f32::max(0.0, velocity_vec.x);
	} else {
		passive_deceleration.x = -velocity_vec.x;
	}

	if keyboard_input.pressed(KeyCode::W) {
		acceleration_vec += Vec2::Y;
		velocity_vec.y = f32::max(0.0, velocity_vec.y);
	} else if keyboard_input.pressed(KeyCode::S) {
		acceleration_vec += Vec2::NEG_Y;
		velocity_vec.y = f32::min(0.0, velocity_vec.y);
	} else {
		passive_deceleration.y = -velocity_vec.y;
	}

	passive_deceleration = passive_deceleration.clamp_length_max(acceleration.rate * 2.0);

	acceleration_vec = acceleration_vec.clamp_length_max(acceleration.rate);
	*velocity_vec = (*velocity_vec + acceleration_vec + passive_deceleration).clamp_length_max(acceleration.max_velocity);
}

pub fn animate_player_sprite(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>), With<Player>>,
) {
	for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
		timer.tick(time.delta());
		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
	let texture_handle = asset_server.load("wizard.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 10, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.spawn((
		Player,
		Velocity(Vec2::new(0.0, 0.0)),
		Acceleration {
			max_velocity: 10.0,
			rate: 2.0,
		},
		SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_translation(Vec3::new(0.0, 0.0, 2.0)),
			..default()
		},
		AnimationTimer(Timer::from_seconds(0.1, TimerMode::Repeating)),
	));
}
