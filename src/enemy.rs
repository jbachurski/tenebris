use std::{f32::consts::TAU, vec::Vec};

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

use crate::mob::*;

const GRADE_VECTORS: usize = 20;

#[derive(Component)]
pub struct EnemySkeleton;

#[derive(Component)]
pub struct EnemyWraith {
	angle: f32,
	angle_vel: f32,
}

enum EnemyGooState {
	Jumping(u32, Vec2),
	Waiting(u32),
}

#[derive(Component)]
pub struct EnemyGoo {
	state: EnemyGooState,
}

pub fn spawn_enemies(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(30., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(3200. + 50.0, 3200. + 100.0, 1.0)),
			..default()
		})
		.insert(EnemySkeleton)
		.insert(Bounded {
			size: Vec2::splat(2. * 30.),
		})
		.insert(Mob { health: 3 })
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(12.0, 12.0));
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(30., 6).into()).into(),
			material: materials.add(ColorMaterial::from(Color::TURQUOISE)),
			transform: Transform::from_translation(Vec3::new(3200. + -20.0, 3200. + -100.0, 1.0)),
			..default()
		})
		.insert(EnemySkeleton)
		.insert(Bounded {
			size: Vec2::splat(2. * 30.),
		})
		.insert(Mob { health: 3 })
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(12.0, 12.0));
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(40., 3).into()).into(),
			material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
			transform: Transform::from_translation(Vec3::new(3200. + 200.0, 3200. + -50.0, 1.0)),
			..default()
		})
		.insert(EnemyWraith {
			angle: 0.0,
			angle_vel: 0.0,
		})
		.insert(Bounded {
			size: Vec2::splat(2. * 30.),
		})
		.insert(Mob { health: 3 })
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		});
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(25., 16).into()).into(),
			material: materials.add(ColorMaterial::from(Color::BLUE)),
			transform: Transform::from_translation(Vec3::new(3200. + -300.0, 3200. + 0.0, 1.0)),
			..default()
		})
		.insert(EnemyGoo {
			state: EnemyGooState::Waiting(0),
		})
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(12.0, 12.0));
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
	mut enemies: Query<(&Transform, &mut Velocity), (With<EnemySkeleton>, Without<Camera>)>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = cameras.single().translation;
	let enemy_positions: Vec<Vec2> = enemies.iter().map(|(t, _)| t.translation.xy()).collect();
	for (enemy_tr, mut velocity) in enemies.iter_mut() {
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

		velocity.linvel = target_v * 2.0 * v_mod * 60.;
	}
}

pub fn run_wraith(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<(&Transform, &mut Velocity, &mut EnemyWraith), Without<Camera>>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = cameras.single().translation;
	for (enemy_tr, mut velocity, mut wraith) in enemies.iter_mut() {
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
		velocity.linvel =
			(3.0 + 1.0 * (1.0 - (angle_diff.abs() / (TAU / 4.0)).min(1.0))) * Vec2::from_angle(wraith.angle) * 60.;
	}
}

pub fn run_goo(
	cameras: Query<&Transform, With<Camera>>,
	mut enemies: Query<(&mut Transform, &mut Velocity, &mut EnemyGoo), Without<Camera>>,
) {
	let camera_pos = cameras.single().translation;
	for (mut enemy_tr, mut velocity, mut goo) in enemies.iter_mut() {
		let diff = camera_pos.xy() - enemy_tr.translation.xy();
		enemy_tr.scale = Vec3::splat(1.0);
		goo.state = match goo.state {
			EnemyGooState::Jumping(ticks, heading) => {
				if ticks > 0 {
					velocity.linvel = heading * 6.0 * 60.;
					EnemyGooState::Jumping(ticks - 1, heading)
				} else {
					velocity.linvel = Vec2::ZERO;
					EnemyGooState::Waiting(75)
				}
			},
			EnemyGooState::Waiting(ticks) => {
				if diff.length() > 600.0 {
					EnemyGooState::Waiting(ticks)
				} else if ticks > 0 {
					if ticks < 10 {
						enemy_tr.scale = Vec3::splat(lerp(0.0, 1.0, 10.0, 0.8, ticks as f32));
					} else if ticks < 50 {
						enemy_tr.scale = Vec3::splat(lerp(10.0, 0.8, 50.0, 1.0, ticks as f32));
					}
					EnemyGooState::Waiting(ticks - 1)
				} else {
					EnemyGooState::Jumping(45, diff.normalize())
				}
			},
		};
	}
}
