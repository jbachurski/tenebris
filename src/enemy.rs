use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use std::{f32::consts::TAU, vec::Vec};

const GRADE_VECTORS: usize = 20;

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

fn dister(heading: Vec2, target: Vec2, d0: f32) -> f32 {
	let a = heading.dot(target.normalize());
	let d = target.length();
	if d < d0 {
		-a * (1.0 - d / d0)
	} else {
		a
	}
}

pub fn run_enemy(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<&mut Transform, (With<Enemy>, Without<Camera>)>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = cameras.single().translation;
	let enemy_positions: Vec<Vec2> = enemies.iter().map(|t| t.translation.xy()).collect();
	for mut enemy_tr in enemies.iter_mut() {
		let mut grade = |v: Vec2| {
			let mut result: f32 = 0.0;
			let pos = enemy_tr.translation.xy();

			result += dister(v, camera_pos.xy() - pos, 80.0);
			for other in enemy_positions.iter() {
				if (*other - pos).length() > 1e-5 {
					result -= 0.5 * dister(v, *other - pos, 30.0);
				}
			}
			lines.line_colored(
				pos.extend(1.0),
				(pos + 100.0 * result.abs() * v).extend(1.0),
				0.0,
				if result < 0.0 { Color::RED } else { Color::GREEN },
			);
			result
		};

		let target_v = (0..GRADE_VECTORS)
			.map(|i| Vec2::from_angle((i as f32) * TAU / (GRADE_VECTORS as f32)))
			.max_by(|v1, v2| grade(*v1).partial_cmp(&grade(*v2)).unwrap())
			.unwrap();

		enemy_tr.translation += (target_v * 2.0).extend(0.0);
	}
}
