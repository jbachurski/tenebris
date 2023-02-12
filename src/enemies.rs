pub mod ranger;
use ranger::*;

pub mod utils;

pub mod wraith;
use wraith::*;

pub mod goo;
use bevy::prelude::*;
use goo::*;

pub fn spawn_random_enemy(
	mut commands: Commands,
	mut meshes: ResMut<Assets<Mesh>>,
	mut materials: ResMut<Assets<ColorMaterial>>,
) {
	let position = Vec2::new(3200. + -20.0, 3200. + -100.0);

	spawn_ranger(&mut commands, &mut meshes, &mut materials, position);
	spawn_wraith(&mut commands, &mut meshes, &mut materials, position);
	spawn_goo(&mut commands, &mut meshes, &mut materials, position);
}
