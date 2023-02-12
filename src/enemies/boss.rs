use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;

use crate::{mob::*, player::*};

pub enum BossState {
	Waiting(f32),
}

#[derive(Component)]
pub struct EnemyBoss {
	pub state: BossState,
}

pub fn spawn_boss(commands: &mut Commands, meshes: &mut ResMut<Assets<Mesh>>, materials: &mut ResMut<Assets<ColorMaterial>>) {
	commands
		.spawn(MaterialMesh2dBundle {
			mesh: meshes.add(shape::RegularPolygon::new(35., 4).into()).into(),
			material: materials.add(ColorMaterial::from(Color::RED)),
			transform: Transform::from_translation(Vec3::new(3200. + 500.0, 3200. + 500.0, 2.0)),
			..default()
		})
		.insert(EnemyBoss {
			state: BossState::Waiting(7.0),
		})
		.insert(Mob { health: 3 })
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(Bounded {
			size: Vec2::splat(2. * 30.),
		})
		.insert(Mob { health: 10 })
		.insert(RigidBody::Dynamic)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(24.0, 24.0));
}

pub fn run_boss(
	time: Res<Time>,
	commands: Commands,
	players: Query<&Transform, With<Player>>,
	mut bosses: Query<(&mut Transform, &mut EnemyBoss), Without<Player>>,
) {
	let camera_pos = players.single().translation;
	for (mut transform, mut boss) in bosses.iter_mut() {}
}
