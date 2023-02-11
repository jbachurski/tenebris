use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::*};

use crate::{enemy::EnemyWraith, tiles::*};

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

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration {
	pub max_velocity: f32,
	pub rate: f32,
}

pub fn move_by_velocity(mut entities: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in entities.iter_mut() {
		transform.translation += velocity.0.extend(0.0);
	}
}

pub fn resolve_collisions(
	tile_manager: Res<TileManager>,
	tiles: Query<&Transform, With<Tile>>,
	mut entities: Query<(&mut Transform, &Bounded), (With<CollidesWithWalls>, Without<Tile>, Without<EnemyWraith>)>,
) {
	for (mut transform, bounded) in entities.iter_mut() {
		for tile_transform in tiles.iter() {
			let (tile_x, tile_y) = position_to_tile_position(&tile_transform.translation.xy()).into();
			if tile_manager.is_wall[tile_x as usize][tile_y as usize] {
				let bounding_box = Rect::from_center_size(transform.translation.xy(), bounded.size);
				let tile_bounding_box = Rect::from_corners(
					tile_transform.translation.xy(),
					tile_transform.translation.xy() + Vec2::new(32., 32.),
				);
				match collide(
					bounding_box.center().extend(0.),
					bounding_box.size(),
					tile_bounding_box.center().extend(0.),
					tile_bounding_box.size(),
				) {
					Some(Collision::Left) => {
						transform.translation.x = tile_bounding_box.min.x - (bounded.size.x / 2.);
					},
					Some(Collision::Right) => {
						transform.translation.x = tile_bounding_box.max.x + (bounded.size.x / 2.);
					},
					Some(Collision::Top) => {
						transform.translation.y = tile_bounding_box.max.y + (bounded.size.y / 2.);
					},
					Some(_) => {
						transform.translation.y = tile_bounding_box.min.y - (bounded.size.y / 2.);
					},
					None => {},
				}
			}
		}
	}
}
