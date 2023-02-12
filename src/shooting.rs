use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{mob::*, player::*, Despawn};

#[derive(Component)]
pub struct Projectile {
	pub damage: i32,
}

#[derive(Component)]
pub struct Firebolt;

pub const FIREBALL_LIFE: f32 = 1.0;
pub const CRYSTAL_LIFE: f32 = 0.6;
pub const MINE_LIFE: f32 = 7.0;

pub const FIREBALL_COOLDOWN: f32 = 0.6;
pub const CRYSTAL_COOLDOWN: f32 = 0.17;
pub const MINE_COOLDOWN: f32 = 2.0;

#[derive(Component)]
pub struct Crystal {
	basevel: Vec2,
	heading: Vec2,
}

#[derive(Component)]
pub struct Mine;

#[derive(Component, Deref, DerefMut)]
pub struct ProjectileTimer(Timer);

pub fn player_shoot(
	commands: Commands,
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	windows: Res<Windows>,
	mouse_button_input: Res<Input<MouseButton>>,
	mut player_query: Query<(
		&mut FireboltCooldownTimer,
		&mut CrystalCooldownTimer,
		&mut MineCooldownTimer,
		&Transform,
		&Velocity,
		&Player,
	)>,
	camera_query: Query<(&Camera, &GlobalTransform)>,
) {
	let (camera, camera_transform) = camera_query.single();
	let (mut firebolt_timer, mut crystal_timer, mut mine_timer, player_transform, player_velocity, player) =
		player_query.single_mut();

	if match player.select {
		PlayerWeaponSelect::Firebolt => {
			firebolt_timer.tick(time.delta());
			!firebolt_timer.finished()
		},
		PlayerWeaponSelect::Crystals => {
			crystal_timer.tick(time.delta());
			!crystal_timer.finished()
		},
		PlayerWeaponSelect::Mine => {
			mine_timer.tick(time.delta());
			!mine_timer.finished()
		},
	} {
		return;
	};

	let window = windows.get_primary().unwrap();
	if let Some(cursor_position) = window.cursor_position() {
		if mouse_button_input.pressed(MouseButton::Left) {
			let cursor_position = get_cursor_world_pos(camera, camera_transform, window, cursor_position);
			cast_spell(
				commands,
				player_transform,
				player_velocity,
				&player,
				asset_server,
				cursor_position,
			);
			match player.select {
				PlayerWeaponSelect::Firebolt => firebolt_timer.reset(),
				PlayerWeaponSelect::Crystals => crystal_timer.reset(),
				PlayerWeaponSelect::Mine => mine_timer.reset(),
			};
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

fn cast_spell(
	mut commands: Commands,
	player_transform: &Transform,
	player_velocity: &Velocity,
	player: &Player,
	asset_server: Res<AssetServer>,
	cursor_position: Vec2,
) {
	let heading = (cursor_position - player_transform.translation.truncate()).normalize();
	match player.select {
		PlayerWeaponSelect::Firebolt => {
			commands.spawn((
				SpriteBundle {
					texture: asset_server.load("fire_bolt.png"),
					transform: *player_transform,
					..default()
				},
				Velocity {
					linvel: heading * 7.0 * 60.,
					angvel: 0.0,
				},
				Projectile { damage: 5 },
				ProjectileTimer(Timer::from_seconds(FIREBALL_LIFE, TimerMode::Once)),
				Bounded { size: Vec2::splat(16.0) },
				RigidBody::Dynamic,
				LockedAxes::ROTATION_LOCKED,
				Firebolt,
			));
		},
		PlayerWeaponSelect::Crystals => {
			commands.spawn((
				SpriteBundle {
					texture: asset_server.load("fire_bolt.png"),
					transform: *player_transform,
					..default()
				},
				Velocity {
					linvel: player_velocity.linvel,
					angvel: 0.0,
				},
				Projectile { damage: 1 },
				ProjectileTimer(Timer::from_seconds(CRYSTAL_LIFE, TimerMode::Once)),
				Bounded { size: Vec2::splat(10.0) },
				RigidBody::Dynamic,
				LockedAxes::ROTATION_LOCKED,
				Crystal {
					basevel: player_velocity.linvel,
					heading,
				},
			));
		},
		PlayerWeaponSelect::Mine => {},
	};
}

pub fn update_crystals_velocity(
	time: Res<Time>,
	mut players: Query<&Player>,
	mut crystals: Query<(&mut ProjectileTimer, &mut Velocity, &Crystal)>,
) {
	let player = players.single();
	for (mut timer, mut velocity, crystal) in crystals.iter_mut() {
		timer.tick(time.delta());
		let t = timer.remaining().as_secs_f32();
		if t > 0.0 {
			velocity.linvel = crystal.basevel + crystal.heading * (t / CRYSTAL_LIFE) * 12.0 * 60.0
		}
	}
}

pub fn despawn_old_projectiles(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut ProjectileTimer)>) {
	for (entity, mut timer) in query.iter_mut() {
		timer.tick(time.delta());
		if timer.finished() {
			commands.entity(entity).insert(Despawn);
		}
	}
}
