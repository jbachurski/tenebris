pub mod ranger;
use ranger::*;

pub mod utils;

pub mod wraith;
use wraith::*;

pub mod boss;
use boss::*;

pub mod goo;
use std::f32::consts::TAU;

use bevy::prelude::*;
use goo::*;
use rand::prelude::*;

use crate::{player::*, tiles::*, tilesim::*};

#[derive(Resource)]
pub struct EnemySpawner {
	pub timer: Timer,
	pub max_enemy_count: i64,
}

#[derive(Component)]
pub struct Enemy;

pub fn spawn_random_enemy(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
	mut spawner: ResMut<EnemySpawner>,
	time: Res<Time>,
	simulator: Res<Simulator>,
	enemy_query: Query<&Enemy>,
	player_query: Query<&Transform, With<Player>>,
) {
	spawner.timer.tick(time.delta());

	if spawner.timer.just_finished() && (enemy_query.iter().len() as i64) < spawner.max_enemy_count {
		let mut rng = rand::thread_rng();
		let player_position = player_query.single().translation.truncate();
		let spawn_distance = simulator.reality_params.0 as f32 * TILE_SIZE as f32;
		let angle = rng.gen_range(0.0..TAU);

		let displacement = Vec2::new(angle.cos(), angle.sin()) * spawn_distance as f32;
		let spawn_position = player_position + displacement;

		let pos = position_to_tile_position(&spawn_position);

		// Enemies failing to spawn sometimes adds randomness and is fine
		if !simulator.in_bounds(pos) || simulator.grid.is_wall[pos.x as usize][pos.y as usize] {
			return;
		}

		let rand_z: f32 = rng.gen();
		let position = spawn_position.extend(1.0 + rand_z);
		match rng.gen_range(0..3) {
			0 => spawn_ranger(&mut commands, &mut meshes, &mut materials, position),
			1 => spawn_wraith(&mut commands, &mut meshes, &mut materials, position),
			_ => spawn_goo(&mut commands, &mut meshes, &mut materials, position),
		}
	}
}

pub fn despawn_far_enemies(
	mut commands: Commands,
	simulator: Res<Simulator>,
	enemy_query: Query<(Entity, &Transform), With<Enemy>>,
	player_query: Query<&Transform, With<Player>>,
) {
	let player_position = player_query.single().translation.truncate();
	let despawn_distance = simulator.reality_params.1 as f32 * TILE_SIZE as f32;

	for (entity, transform) in enemy_query.iter() {
		let position = transform.translation.truncate();
		if (player_position - position).length() > despawn_distance {
			commands.entity(entity).despawn();
		}
	}
}
