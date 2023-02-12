use std::f32::consts::TAU;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

use crate::{mob::*, player::*};

#[derive(Component)]
pub struct EnemyWraith {
	angle: f32,
	angle_vel: f32,
}

pub fn spawn_wraith(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	position: Vec3,
) {
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(40., 3).into()).into(),
			material: materials.add(ColorMaterial::from(Color::ORANGE_RED)),
			transform: Transform::from_translation(position),
			..default()
		},
		EnemyWraith {
			angle: 0.0,
			angle_vel: 0.0,
		},
		Mob { health: 3 },
		Bounded {
			size: Vec2::splat(2. * 20.),
		},
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
		Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		},
	));
}

pub fn run_wraith(
	time: Res<Time>,
	players: Query<&Transform, With<Player>>,
	mut enemies: Query<(&Transform, &mut Velocity, &mut EnemyWraith), Without<Player>>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = players.single().translation;
	for (enemy_tr, mut velocity, mut wraith) in enemies.iter_mut() {
		let angle_diff = Vec2::from_angle(wraith.angle).angle_between(camera_pos.xy() - enemy_tr.translation.xy());

		wraith.angle_vel *= f32::powf(0.5, time.delta_seconds());
		wraith.angle_vel += (angle_diff / 3.0).clamp(-TAU / 1024.0, TAU / 1024.0);
		wraith.angle_vel = wraith.angle_vel.clamp(-TAU / 256.0, TAU / 256.0);
		wraith.angle += wraith.angle_vel * time.delta_seconds() * 60.0;
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
