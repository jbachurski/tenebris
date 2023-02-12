use std::{f32::consts::TAU, vec::Vec};

use bevy::{math::Vec3Swizzles, prelude::*};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

use super::{utils::*, Enemy};
use crate::{mob::*, player::*};

const GRADE_VECTORS: usize = 20;

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

#[derive(Component)]
pub struct EnemyRanger {
	ticks: f32,
}

#[derive(Component, Deref, DerefMut)]
pub struct RangerAnimationTimer(Timer);

pub fn spawn_ranger(
	commands: &mut Commands,
	asset_server: &mut Res<AssetServer>,
	texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
	position: Vec3,
) {
	let texture_handle = asset_server.load("ranger.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 10, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.spawn((
		SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_translation(position),
			..default()
		},
		RangerAnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
		EnemyRanger { ticks: 3.0 },
		Bounded {
			size: Vec2::splat(2. * 20.),
		},
		Mob { health: 20 },
		Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		},
		RigidBody::Dynamic,
		LockedAxes::ROTATION_LOCKED,
		CollidesWithWalls,
		Collider::cuboid(12.0, 12.0),
		Enemy,
		SpriteFacingMovement,
		PlayerDanger {
			damage: 1,
			hit_despawn: false,
			til_despawn: f32::INFINITY,
		},
	));
}

pub fn run_ranger(
	mut commands: Commands,
	time: Res<Time>,
	asset_server: Res<AssetServer>,
	players: Query<&Transform, With<Player>>,
	mut enemies: Query<(&Transform, &mut Velocity, &mut EnemyRanger), Without<Player>>,
	mut lines: ResMut<DebugLines>,
) {
	let camera_pos = players.single().translation.xy();
	let enemy_positions: Vec<Vec2> = enemies.iter().map(|(t, _, _)| t.translation.xy()).collect();
	for (enemy_tr, mut velocity, mut ranger) in enemies.iter_mut() {
		let pos = enemy_tr.translation.xy();
		let mut grade = |v: Vec2| {
			let mut result: f32 = 0.0;

			result += dister(v, camera_pos - pos, 150.0);
			for other in enemy_positions.iter() {
				if (*other - pos).length() > 1e-5 {
					result -= 0.5 * dister(v, *other - pos, 30.0);
				}
			}
			// lines.line_colored(
			// 	pos.extend(1.0),
			// 	(pos + 100.0 * result.abs() * v).extend(1.0),
			// 	0.0,
			// 	if result < 0.0 { Color::RED } else { Color::GREEN },
			// );
			result
		};

		let target_v = best_heading(GRADE_VECTORS, &mut grade);
		let v_mod = (grade(target_v) / 0.7).clamp(0.0, 1.0).sqrt();

		velocity.linvel = target_v * 2.0 * v_mod * 60.;

		let t = ranger.ticks - time.delta().as_secs_f32();

		if t <= 0.0 {
			commands
				.spawn(SpriteBundle {
					texture: asset_server.load("fire_bolt.png"),
					transform: Transform::from_translation(pos.extend(2.0)),
					..default()
				})
				.insert(PlayerDanger {
					damage: 1,
					hit_despawn: true,
					til_despawn: 1.5,
				})
				.insert(Bounded { size: Vec2::splat(10.0) })
				.insert(RigidBody::Dynamic)
				.insert(Velocity {
					linvel: (camera_pos - pos).normalize() * 500.0,
					angvel: 0.0,
				});
			ranger.ticks = 2.0;
		} else {
			ranger.ticks = t;
		}
	}
}

pub fn animate_ranger_sprite(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(
		&mut RangerAnimationTimer,
		&mut TextureAtlasSprite,
		&Handle<TextureAtlas>,
		&Velocity,
	)>,
) {
	for (mut timer, mut sprite, texture_atlas_handle, velocity) in &mut query {
		timer.tick(time.delta());
		if timer.just_finished() {
			let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
			if velocity.linvel.length() < 1e-5 {
				sprite.index = 20
			} else {
				sprite.index = 20 + (i32::max(0, sprite.index as i32 - 19) as usize % 10);
			}
		}
	}
}
