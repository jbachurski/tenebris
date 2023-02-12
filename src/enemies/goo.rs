use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{mob::*, player::*};
use super::{utils::*, Enemy};

enum EnemyGooState {
	Jumping(f32, Vec2),
	Waiting(f32),
}

#[derive(Component)]
pub struct EnemyGoo {
	state: EnemyGooState,
}

pub fn spawn_goo(
	commands: &mut Commands,
	meshes: &mut ResMut<Assets<Mesh>>,
	materials: &mut ResMut<Assets<ColorMaterial>>,
	position: Vec3,
) {
	commands.spawn((
		MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(25., 16).into()).into(),
			material: materials.add(ColorMaterial::from(Color::BLUE)),
			transform: Transform::from_translation(position),
			..default()
		},
		EnemyGoo {
			state: EnemyGooState::Waiting(0.0),
		},
		Mob { health: 3 },
		Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		},
		Bounded {
			size: Vec2::splat(2. * 20.),
		},
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
		CollidesWithWalls,
		Collider::cuboid(12.0, 12.0),
		Enemy,
	));
}

pub fn run_goo(
	time: Res<Time>,
	players: Query<&Transform, With<Player>>,
	mut enemies: Query<(&mut Transform, &mut Velocity, &mut EnemyGoo), Without<Player>>,
) {
	let camera_pos = players.single().translation;
	for (mut enemy_tr, mut velocity, mut goo) in enemies.iter_mut() {
		let diff = camera_pos.xy() - enemy_tr.translation.xy();
		enemy_tr.scale = Vec3::splat(1.0);
		goo.state = match goo.state {
			EnemyGooState::Jumping(ticks, heading) => {
				if ticks > 0.0 {
					velocity.linvel = heading * 6.0 * 60.;
					EnemyGooState::Jumping(ticks - time.delta_seconds() * 60.0, heading)
				} else {
					velocity.linvel = Vec2::ZERO;
					EnemyGooState::Waiting(75.0)
				}
			},
			EnemyGooState::Waiting(ticks) => {
				if diff.length() > 600.0 {
					EnemyGooState::Waiting(ticks)
				} else if ticks > 0.0 {
					if ticks < 10.0 {
						enemy_tr.scale = Vec3::splat(lerp(0.0, 1.0, 10.0, 0.8, ticks as f32));
					} else if ticks < 50.0 {
						enemy_tr.scale = Vec3::splat(lerp(10.0, 0.8, 50.0, 1.0, ticks as f32));
					}
					EnemyGooState::Waiting(ticks - time.delta_seconds() * 60.0)
				} else {
					EnemyGooState::Jumping(45.0, diff.normalize())
				}
			},
		};
	}
}
