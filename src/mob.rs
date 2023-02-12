use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::Velocity;
use rand::{thread_rng, Rng};

use crate::{
	gems::{spawn_gems, DropsGems},
	player::*,
	shooting::Projectile,
	Despawn,
};

pub const VELOCITY_ERROR: f32 = 1.0; // Rangers keep moving a lot for some reason

#[derive(Component)]
pub struct Bounded {
	pub size: Vec2, // Radius of the bounding box.
}

#[derive(Component)]
pub struct CollidesWithWalls;

#[derive(Component)]
pub struct Mob {
	pub health: i32,
}

#[derive(Component)]
pub struct PlayerDanger {
	pub damage: i32,
	pub hit_despawn: bool,
	pub til_despawn: f32,
}

pub fn projectile_hit_mobs(
	mut commands: Commands,
	mut projectiles: Query<(Entity, &Transform, &Bounded, &Projectile), Without<Despawn>>,
	mut mobs: Query<(&Transform, &Bounded, &mut Mob)>,
) {
	for (proj_entity, proj_transform, proj_bound, proj) in projectiles.iter_mut() {
		let proj_rect = Rect::from_center_size(proj_transform.translation.xy(), proj_bound.size);
		for (mob_transform, mob_bound, mut mob) in mobs.iter_mut() {
			let mob_rect = Rect::from_center_size(mob_transform.translation.xy(), mob_bound.size);
			if !proj_rect.intersect(mob_rect).is_empty() {
				mob.health -= proj.damage;
				commands.entity(proj_entity).insert(Despawn);
				break;
			}
		}
	}
}

pub fn danger_hit_player(
	time: Res<Time>,
	mut commands: Commands,
	mut mobs: Query<(Entity, &Transform, &Bounded, &mut PlayerDanger), Without<Player>>,
	mut players: Query<(&Transform, &Bounded, &mut Player)>,
) {
	let (transform, bound, mut player) = players.single_mut();
	let rect = Rect::from_center_size(transform.translation.xy(), bound.size);
	for (entity, mob_transform, mob_bound, mut danger) in mobs.iter_mut() {
		danger.til_despawn -= time.delta().as_secs_f32();
		let mob_rect = Rect::from_center_size(mob_transform.translation.xy(), mob_bound.size);
		if !rect.intersect(mob_rect).is_empty() {
			player.take_damage(danger.damage);
			if danger.hit_despawn {
				commands.entity(entity).insert(Despawn);
			}
			break;
		}
		if danger.til_despawn < 0.0 {
			commands.entity(entity).insert(Despawn);
		}
	}
}

#[derive(Component)]
pub struct SpriteFacingMovement;

pub fn mob_face_movement_sprite_sheet(mut mob_query: Query<(&mut TextureAtlasSprite, &Velocity), With<SpriteFacingMovement>>) {
	for (mut sprite, velocity) in mob_query.iter_mut() {
		// Want to avoid flipping when x = 0
		if velocity.linvel.x < -VELOCITY_ERROR {
			sprite.flip_x = true;
		}
		if velocity.linvel.x > VELOCITY_ERROR {
			sprite.flip_x = false;
		}
	}
}

pub fn mob_face_movement_sprite(mut mob_query: Query<(&mut Sprite, &Velocity), With<SpriteFacingMovement>>) {
	for (mut sprite, velocity) in mob_query.iter_mut() {
		// Want to avoid flipping when x = 0
		if velocity.linvel.x < -VELOCITY_ERROR {
			sprite.flip_x = true;
		}
		if velocity.linvel.x > VELOCITY_ERROR {
			sprite.flip_x = false;
		}
	}
}

pub fn unspawn_dead_mobs(
	mut commands: Commands,
	mut asset_server: Res<AssetServer>,
	mobs: Query<(Entity, &Transform, &Mob, &DropsGems), Without<Player>>,
) {
	for (entity, transform, mob, gem_dist) in mobs.iter() {
		let mut rng = thread_rng();
		let gem_count = gem_dist.0 + rng.gen_range(0..gem_dist.1);

		if mob.health <= 0 {
			commands.entity(entity).insert(Despawn);
			spawn_gems(&mut commands, &mut asset_server, gem_count, transform.translation.truncate())
		}
	}
}

#[derive(Component)]
pub struct Acceleration {
	pub max_velocity: f32,
	pub rate: f32,
}
