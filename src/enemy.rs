use std::f32::consts::TAU;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};

const GRADE_VECTORS: usize = 8;

#[derive(Component)]
pub struct Enemy;

pub fn spawn_enemy(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(50.0, 100.0, 1.0)),
			..default()
		})
		.insert(Enemy);
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(50., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(-20.0, -100.0, 1.0)),
			..default()
		})
		.insert(Enemy);
}

pub fn run_enemy(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<(&mut Transform, Entity), (With<Enemy>, Without<Camera>)>,
) {
	let camera_pos = cameras.single().translation;
	for (mut enemy_tr, _enemy) in enemies.iter_mut() {
		// for i in 0..GRADE_VECTORS {
		//     let mut grade: f32 = 0.0;
		//     let v = Vec2::from_angle(TAU);

		//     for (other_pos, other) in enemies.iter_mut() {

		//     }
		// }
		let diff = enemy_tr.translation.xy() - camera_pos.xy();
		enemy_tr.translation -= (diff * 0.01).extend(0.0);
	}
}
