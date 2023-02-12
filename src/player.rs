use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::mob::*;

#[derive(Component)]
pub struct Player;

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct ShootingTimer(Timer);

#[derive(Default, Resource)]
pub struct MovementPrecedence {
	pub up_has_precedence: Option<bool>,
	pub right_has_precedence: Option<bool>,
}

pub fn update_velocity(
	mut movement_precedence: Local<MovementPrecedence>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Velocity, &Acceleration), With<Player>>,
) {
	let (mut velocity, acceleration) = query.single_mut();
	let velocity_vec = &mut velocity.linvel;

	let mut acceleration_vec = Vec2::ZERO;
	let mut passive_deceleration = Vec2::ZERO;

	if keyboard_input.just_pressed(KeyCode::A) {
		movement_precedence.right_has_precedence = Some(false);
	} else if keyboard_input.just_released(KeyCode::A) {
		movement_precedence.right_has_precedence = if keyboard_input.pressed(KeyCode::D) {
			Some(true)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::D) {
		movement_precedence.right_has_precedence = Some(true);
	} else if keyboard_input.just_released(KeyCode::D) {
		movement_precedence.right_has_precedence = if keyboard_input.pressed(KeyCode::A) {
			Some(false)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::S) {
		movement_precedence.up_has_precedence = Some(false);
	} else if keyboard_input.just_released(KeyCode::S) {
		movement_precedence.up_has_precedence = if keyboard_input.pressed(KeyCode::W) {
			Some(true)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::W) {
		movement_precedence.up_has_precedence = Some(true);
	} else if keyboard_input.just_released(KeyCode::W) {
		movement_precedence.up_has_precedence = if keyboard_input.pressed(KeyCode::S) {
			Some(false)
		} else {
			None
		};
	}

	if movement_precedence.right_has_precedence == Some(false) {
		acceleration_vec += Vec2::NEG_X;
		velocity_vec.x = f32::min(0.0, velocity_vec.x);
	} else if movement_precedence.right_has_precedence == Some(true) {
		acceleration_vec += Vec2::X;
		velocity_vec.x = f32::max(0.0, velocity_vec.x);
	} else {
		passive_deceleration.x = -velocity_vec.x;
	}

	if movement_precedence.up_has_precedence == Some(true) {
		acceleration_vec += Vec2::Y;
		velocity_vec.y = f32::max(0.0, velocity_vec.y);
	} else if movement_precedence.up_has_precedence == Some(false) {
		acceleration_vec += Vec2::NEG_Y;
		velocity_vec.y = f32::min(0.0, velocity_vec.y);
	} else {
		passive_deceleration.y = -velocity_vec.y;
	}

	acceleration_vec *= 60.;

	passive_deceleration = passive_deceleration.clamp_length_max(acceleration.rate * 2.0);

	*velocity_vec = (*velocity_vec + acceleration_vec + passive_deceleration).clamp_length_max(acceleration.max_velocity);
}

pub fn animate_player_sprite(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &Velocity), With<Player>>,
) {
	for (mut timer, mut sprite, texture_atlas_handle, velocity) in &mut query {
		timer.tick(time.delta());
		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			if (velocity.linvel.length() < 1e-5) {
				sprite.index = 20
			} else {
				sprite.index = 20 + (i32::max(0, sprite.index as i32 - 19) as usize % 10);
			}
		}
	}
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
	let texture_handle = asset_server.load("wizard.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 10, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.spawn((
		Player,
		Velocity::default(),
		Acceleration {
			max_velocity: 10.0 * 60.,
			rate: 2.0 * 60.,
		},
		SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_translation(Vec3::new(3200.0, 3200.0, 2.0)),
			..default()
		},
		AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
		Bounded {
			size: Vec2::new(32., 32.),
		},
		CollidesWithWalls,
		ShootingTimer(Timer::from_seconds(0.5, TimerMode::Repeating)),
		RigidBody::Dynamic,
		Collider::cuboid(12.0, 12.0),
		Ccd::enabled(),
		LockedAxes::ROTATION_LOCKED,
		SpriteFacingMovement,
	));
}
