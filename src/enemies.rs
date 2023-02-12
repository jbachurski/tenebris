pub mod ranger;
use ranger::*;

pub mod utils;

pub mod wraith;
use wraith::*;

pub mod goo;
use bevy::prelude::*;
use goo::*;

pub fn spawn_enemies(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	spawn_rangers(&mut commands, &mut meshes, &mut materials);
	spawn_wraiths(&mut commands, &mut meshes, &mut materials);
	spawn_goos(&mut commands, &mut meshes, &mut materials);
}
