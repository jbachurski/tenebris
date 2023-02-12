use std::f32::consts::TAU;

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{mob::*, player::*, Despawn};

#[derive(Component)]
pub struct Gem;

#[derive(Component)]
pub struct DropsGems(pub i64, pub i64);

pub fn spawn_gems(commands: &mut Commands, asset_server: &mut Res<AssetServer>, count: i64, position: Vec2) {
	let mut rng = rand::thread_rng();
	let rand_z: f32 = rng.gen_range(1.0..2.0);

	for _ in 0..count {
		let spawn_distance = rng.gen_range(0.0..30.0);
		let angle = rng.gen_range(0.0..TAU);

		let displacement = Vec2::new(angle.cos(), angle.sin()) * spawn_distance;

		commands.spawn((
			SpriteBundle {
				texture: asset_server.load(format!("gem/gem{}.png", rng.gen_range(1..5))),
				transform: Transform::from_translation((position + displacement).extend(rand_z)),
				..default()
			},
			Bounded { size: Vec2::splat(20.) },
			RigidBody::Dynamic,
			LockedAxes::ROTATION_LOCKED,
			Gem,
		));
	}
}

pub fn player_collect_gem(
	mut commands: Commands,
	mut gem_query: Query<(Entity, &Transform, &Bounded), With<Gem>>,
	mut player_query: Query<(&Transform, &Bounded, &mut Player)>,
) {
	let (player_transform, player_bound, mut player) = player_query.single_mut();

	for (gem_entity, gem_transform, gem_bound) in gem_query.iter_mut() {
		let gem_rect = Rect::from_center_size(gem_transform.translation.xy(), gem_bound.size);
		let player_rect = Rect::from_center_size(player_transform.translation.xy(), player_bound.size);
		if !gem_rect.intersect(player_rect).is_empty() {
			player.gem_count += 1;
			commands.entity(gem_entity).insert(Despawn);
		}
	}
}
