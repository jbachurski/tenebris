use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use std::{f32::consts::TAU, vec::Vec};

const GRADE_VECTORS: usize = 20;

#[derive(Component)]
pub struct EnemySkeleton;

#[derive(Component)]
pub struct EnemyWraith {
	angle: f32,
	angle_vel: f32,
}

pub fn spawn_enemies(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(30., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(50.0, 100.0, 1.0)),
			..default()
		})
		.insert(EnemySkeleton);
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(30., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(-20.0, -100.0, 1.0)),
			..default()
		})
		.insert(EnemySkeleton);
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(40., 3).into()).into(),
			material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
			transform: Transform::from_translation(Vec3::new(200.0, -50.0, 1.0)),
			..default()
		})
		.insert(EnemyWraith {
			angle: 0.0,
			angle_vel: 0.0,
		});
}

fn lerp(x1: f32, y1: f32, x2: f32, y2: f32, x: f32) -> f32 {
	((x - x1) * y2 + (x2 - x) * y1) / (x2 - x1)
}

fn dister(heading: Vec2, target: Vec2, d0: f32) -> f32 {
	let a = heading.dot(target.normalize());
	let d = target.length();
	a * (if d < d0 {
		lerp(0.0, -1.0, d0, 0.0, d)
	} else if d < 3.0 * d0 {
		lerp(d0, 0.0, 3.0 * d0, 1.0, d)
	} else if d < 6.0 * d0 {
		lerp(3.0 * d0, 1.0, 6.0 * d0, 0.0, d)
	} else {
		0.0
	})
}

fn best_heading<F: FnMut(Vec2) -> f32>(n: usize, mut grade: F) -> Vec2 {
	(0..n)
		.map(|i| Vec2::from_angle((i as f32) * TAU / (GRADE_VECTORS as f32)))
		.max_by(|v1, v2| grade(*v1).partial_cmp(&grade(*v2)).unwrap())
		.unwrap()
}

pub fn run_skeleton(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<&mut Transform, (With<EnemySkeleton>, Without<Camera>)>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = cameras.single().translation;
	let enemy_positions: Vec<Vec2> = enemies.iter().map(|t| t.translation.xy()).collect();
	for mut enemy_tr in enemies.iter_mut() {
		let mut grade = |v: Vec2| {
			let mut result: f32 = 0.0;
			let pos = enemy_tr.translation.xy();

			result += dister(v, camera_pos.xy() - pos, 150.0);
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

		let target_v = best_heading(GRADE_VECTORS, &mut grade);
		let v_mod = (grade(target_v) / 0.7).clamp(0.0, 1.0).sqrt();

		enemy_tr.translation += (target_v * 2.0 * v_mod).extend(0.0);
	}
}

pub fn run_wraith(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<(&mut Transform, &mut EnemyWraith), (With<EnemyWraith>, Without<Camera>)>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = cameras.single().translation;
	for (mut enemy_tr, mut wraith) in enemies.iter_mut() {
		let angle_diff = Vec2::from_angle(wraith.angle).angle_between(camera_pos.xy() - enemy_tr.translation.xy());

		wraith.angle_vel += (angle_diff / 3.0).clamp(-TAU / 1024.0, TAU / 1024.0);
		wraith.angle_vel = wraith.angle_vel.clamp(-TAU / 256.0, TAU / 256.0);
		wraith.angle += wraith.angle_vel;
		lines.line_colored(
			enemy_tr.translation,
			enemy_tr.translation + (Vec2::from_angle(wraith.angle) * 70.0).extend(1.0),
			0.0,
			Color::YELLOW,
		);
		enemy_tr.translation +=
			(3.0 + 1.0 * (1.0 - (angle_diff.abs() / (TAU / 4.0)).min(1.0))) * Vec2::from_angle(wraith.angle).extend(0.0);
	}
}
