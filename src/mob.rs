use bevy::{math::Vec3Swizzles, prelude::*};

use crate::{player::*, shooting::Projectile, Despawn};

#[derive(Component)]
pub struct Bounded {
	pub size: Vec2, // Radius of the bounding box.
}

#[derive(Component)]
pub struct CollidesWithWalls;

#[derive(Component)]
pub struct Mob {
	pub health: u32,
}

pub fn projectile_hit_mobs(
	mut commands: Commands,
	mut projectiles: Query<(Entity, &Transform, &Bounded, &Projectile)>,
	mut mobs: Query<(&Transform, &Bounded, &mut Mob), Without<Player>>,
) {
	for (proj_entity, proj_transform, proj_bound, proj) in projectiles.iter_mut() {
		let proj_rect = Rect::from_center_size(proj_transform.translation.xy(), proj_bound.size);
		for (mob_transform, mob_bound, mut mob) in mobs.iter_mut() {
			let mob_rect = Rect::from_center_size(mob_transform.translation.xy(), mob_bound.size);
			if !proj_rect.intersect(mob_rect).is_empty() {
				mob.health = mob.health.saturating_sub(proj.damage);
				commands.entity(proj_entity).insert(Despawn);
				break;
			}
		}
	}
}

pub fn unspawn_dead_mobs(mut commands: Commands, mobs: Query<(Entity, &Mob), Without<Player>>) {
	for (entity, mob) in mobs.iter() {
		if (mob.health == 0) {
			commands.entity(entity).insert(Despawn);
		}
	}
}

#[derive(Component)]
pub struct Acceleration {
	pub max_velocity: f32,
	pub rate: f32,
}
