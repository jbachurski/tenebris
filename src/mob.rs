use bevy::{math::Vec3Swizzles, prelude::*, sprite::collide_aabb::*};

use crate::{enemy::EnemyWraith, tiles::*, tilesim::*};

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
	simulator: Res<Simulator>,
	tiles: Query<&Transform, With<Tile>>,
	mut entities: Query<(&mut Transform, &mut Velocity, &Bounded), (With<CollidesWithWalls>, Without<Tile>)>,
) {
	for (mut transform, mut velocity, bounded) in entities.iter_mut() {
		let projected_translation = transform.translation + velocity.0.extend(0.0);

		for tile_transform in tiles.iter() {
			let (tile_x, tile_y) = position_to_tile_position(&tile_transform.translation.xy()).into();
			if simulator.grid.is_wall[tile_x as usize][tile_y as usize] {
				match collide(
					projected_translation,
					bounded.size,
					tile_transform.translation,
					Vec2::new(32., 32.),
				) {
					Some(_) => {
						velocity.0 = Vec2::ZERO;
					},
					None => {},
				}
			}
		}
	}
}
