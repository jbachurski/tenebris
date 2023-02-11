use bevy::prelude::*;
use crate::mob::*;
use crate::player::*;

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileTimer(Timer);

pub fn player_shoot(
	mut commands: Commands,
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	windows: Res<Windows>,
	mouse_button_input: Res<Input<MouseButton>>,
	mut query: Query<(&mut ShootingTimer, &Transform, &Velocity), With<Player>>,
	q_camera: Query<(&Camera, &GlobalTransform)>,
) {
	let (mut timer, transform, velocity) = query.single_mut();
	let (camera, camera_transform) = q_camera.single();

	timer.tick(time.delta());

	let window = windows.get_primary().unwrap();
	if let Some(position) = window.cursor_position() {
		if timer.just_finished() && mouse_button_input.pressed(MouseButton::Left) {
			let window_size = Vec2::new(window.width() as f32, window.height() as f32);

			// convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
			let ndc = (position / window_size) * 2.0 - Vec2::ONE;

			// matrix for undoing the projection and camera transform
			let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

			// use it to convert ndc to world-space coordinates
			let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

			// reduce it to a 2D value
			let world_pos: Vec2 = world_pos.truncate();

			eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);

			commands.spawn((
				SpriteBundle {
					texture: asset_server.load("fire_bolt.png"),
					transform: *transform,
					..default()
				},
				Velocity(velocity.0 + (world_pos - transform.translation.truncate()).normalize() * 10.0),
				ProjectileTimer(Timer::from_seconds(1.0, TimerMode::Once)),
			));
			// let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			// sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
		}
	}
}

pub fn despawn_old_projectiles(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut ProjectileTimer)>) {
	for (entity, mut timer) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			commands.entity(entity).despawn();
		}
	}
}
