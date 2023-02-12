use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use super::{ranger::*, utils::*, wraith::*};
use crate::{mob::*, player::*};

enum EnemyGooState {
	Jumping(f32, Vec2),
	Waiting(f32),
}

#[derive(Component)]
pub struct EnemyGoo {
	state: EnemyGooState,
}

pub fn spawn_enemies(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
	spawn_rangers(&mut commands, &mut meshes, &mut materials);
	spawn_wraiths(&mut commands, &mut meshes, &mut materials);
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(25., 16).into()).into(),
			material: materials.add(ColorMaterial::from(Color::BLUE)),
			transform: Transform::from_translation(Vec3::new(3200. + -300.0, 3200. + 0.0, 1.0)),
			..default()
		})
		.insert(EnemyGoo {
			state: EnemyGooState::Waiting(0.0),
		})
		.insert(Mob { health: 3 })
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(Bounded {
			size: Vec2::splat(2. * 20.),
		})
		.insert(Mob { health: 3 })
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(12.0, 12.0));
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
