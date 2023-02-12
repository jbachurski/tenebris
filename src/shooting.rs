use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{mob::*, player::*};

#[derive(Component)]
pub struct Projectile {
	pub damage: u32,
}

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileTimer(Timer);

pub fn player_shoot(
	commands: Commands,
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	windows: Res<Windows>,
	mouse_button_input: Res<Input<MouseButton>>,
	mut player_query: Query<(&mut ShootingTimer, &Transform, &Velocity), With<Player>>,
	camera_query: Query<(&Camera, &GlobalTransform)>,
) {
	let (camera, camera_transform) = camera_query.single();
	let (mut shooting_timer, player_transform, player_velocity) = player_query.single_mut();

	// Tick the timer only if shooting is on cooldown, or the player is trying to shoot
	if !shooting_timer.just_finished() {
		shooting_timer.tick(time.delta());
	} else {
		let window = windows.get_primary().unwrap();
		if let Some(cursor_position) = window.cursor_position() {
			if mouse_button_input.pressed(MouseButton::Left) {
				shooting_timer.tick(time.delta());

				let cursor_position = get_cursor_world_pos(camera, camera_transform, window, cursor_position);
				shoot_projectile(commands, player_transform, player_velocity, asset_server, cursor_position)
			}
		}
	}
}

fn get_cursor_world_pos(camera: &Camera, camera_transform: &GlobalTransform, window: &Window, cursor_position: Vec2) -> Vec2 {
	let window_size = Vec2::new(window.width() as f32, window.height() as f32);

	// convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
	let ndc = (cursor_position / window_size) * 2.0 - Vec2::ONE;

	// matrix for undoing the projection and camera transform
	let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();

	// use it to convert ndc to world-space coordinates
	let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));

	// reduce it to a 2D value
	return world_pos.truncate();
}

fn shoot_projectile(
	mut commands: Commands,
	player_transform: &Transform,
	player_velocity: &Velocity,
	asset_server: Res<AssetServer>,
	cursor_position: Vec2,
) {
	commands.spawn((
		SpriteBundle {
			texture: asset_server.load("fire_bolt.png"),
			transform: *player_transform,
			..default()
		},
		Velocity {
			linvel: (cursor_position - player_transform.translation.truncate()).normalize() * 10.0 * 60.,
			angvel: 0.0,
		},
		Projectile { damage: 1 },
		ProjectileTimer(Timer::from_seconds(1.0, TimerMode::Once)),
		Bounded { size: Vec2::splat(16.0) },
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
	));
}

pub fn despawn_old_projectiles(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut ProjectileTimer)>) {
	for (entity, mut timer) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			commands.entity(entity).despawn();
		}
	}
}
