use bevy::prelude::*;
use crate::player::*;

pub fn update_camera(
	mut camera_position_current: Local<Vec2>,
	mut camera_position_desired: Local<Vec2>,
	mut camera: Query<&mut Transform, With<Camera>>,
	player: Query<&Transform, (With<Player>, Without<Camera>)>,
	keycode: Res<Input<KeyCode>>,
) {
	let mut camera = camera.single_mut();
	let player = player.single();


	camera.translation = player.translation
}

pub fn setup_camera(commands: &mut Commands) {
	commands.spawn(Camera2dBundle::default());
}
