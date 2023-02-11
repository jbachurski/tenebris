use bevy::prelude::*;
pub fn update_camera(
	mut camera_position_current: Local<Vec2>,
	mut camera_position_desired: Local<Vec2>,
	mut cameras: Query<&mut Transform, With<Camera>>,
	keycode: Res<Input<KeyCode>>,
) {
	let delta = 8.0;

	// if keycode.pressed(KeyCode::W) {
	// 	camera_position_desired.y += delta;
	// }
	// if keycode.pressed(KeyCode::S) {
	// 	camera_position_desired.y += -delta;
	// }
	// if keycode.pressed(KeyCode::A) {
	// 	camera_position_desired.x += -delta;
	// }
	// if keycode.pressed(KeyCode::D) {
	// 	camera_position_desired.x += delta;
	// }

	let delta = (*camera_position_desired - *camera_position_current) * 0.2;
	*camera_position_current += delta;

	for mut camera in cameras.iter_mut() {
		camera.translation.x = camera_position_current.x;
		camera.translation.y = camera_position_current.y;
	}
}
