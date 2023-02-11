use crate::player::*;
use bevy::{math::Vec3Swizzles, prelude::*};

pub fn update_camera(
	mut camera: Query<&mut Transform, With<Camera>>,
	player: Query<&Transform, (With<Player>, Without<Camera>)>,
) {
	let mut camera = camera.single_mut();
	let player = player.single();

	camera.translation = player.translation.xy().extend(999.0);
}

pub fn setup_camera(commands: &mut Commands) {
	commands.spawn(Camera2dBundle::default());
}
