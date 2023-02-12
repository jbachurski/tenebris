pub mod ranger;
use ranger::*;

pub mod utils;

pub mod wraith;
use wraith::*;

pub mod goo;
use bevy::prelude::*;
use goo::*;
use rand::prelude::*;

pub fn spawn_random_enemy(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let mut rng = rand::thread_rng();

	let rand_z: f32 = rng.gen();
	let position = Vec3::new(3200. + -20.0, 3200. + -100.0, 1.0 + rand_z);

	match rng.gen_range(0..3) {
		0 => spawn_ranger(&mut commands, &mut meshes, &mut materials, position),
		1 => spawn_wraith(&mut commands, &mut meshes, &mut materials, position),
		_ => spawn_goo(&mut commands, &mut meshes, &mut materials, position),
	}
}
