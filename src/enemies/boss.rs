use std::f32::consts::TAU;

use bevy::{math::Vec3Swizzles, prelude::*, sprite::MaterialMesh2dBundle};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::{mob::*, player::*, tiles::position_to_tile_position, utils::*, Simulator};

pub enum BossState {
	Waiting(f32),
	Teleporting(Vec2, f32, i32),
	ChargingCircle(f32),
	RunningCircle(f32, u32),
}

#[derive(Component)]
pub struct EnemyBoss {
	pub state: BossState,
}

const BOSS_WAIT_TO_TELEPORT: f32 = 4.0;
const BOSS_TELEPORT_FRAME: f32 = 0.35;
const BOSS_CHARGE_TO_CIRCLE: f32 = 3.0;
const CIRCLE_ATTACK_TICK: f32 = 0.5;
const CIRCLE_ATTACK_TICKS: u32 = 20;

pub fn spawn_boss(mut commands: &mut Commands, asset_server: &Res<AssetServer>) {
	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("spider.png"),
			transform: Transform::from_translation(Vec3::new(3200. + 500.0, 3200. + 500.0, 2.0))
				.with_scale(Vec3::new(3.0, 3.0, 1.0)),
			..default()
		})
		.insert(EnemyBoss {
			state: BossState::Waiting(BOSS_WAIT_TO_TELEPORT),
		})
		.insert(Mob { health: 300 })
		.insert(Velocity {
			linvel: Vec2::ZERO,
			angvel: 0.0,
		})
		.insert(Bounded {
			size: Vec2::splat(2. * 30.),
		})
		.insert(RigidBody::Fixed)
		.insert(LockedAxes::ROTATION_LOCKED)
		.insert(CollidesWithWalls)
		.insert(Collider::cuboid(24.0, 24.0))
		.insert(Dominance::group(10))
		.insert(PlayerDanger {
			damage: 1,
			hit_despawn: false,
			til_despawn: f32::INFINITY,
		});
}

pub fn boss_shoot(commands: &mut Commands, asset_server: &Res<AssetServer>, source: Vec2, angle: f32) {
	commands
		.spawn(SpriteBundle {
			texture: asset_server.load("cobweb.png"),
			transform: Transform::from_translation(source.extend(2.0)),
			..default()
		})
		.insert(PlayerDanger {
			damage: 1,
			hit_despawn: true,
			til_despawn: 1.0,
		})
		.insert(Bounded { size: Vec2::splat(10.0) })
		.insert(RigidBody::Dynamic)
		.insert(Velocity {
			linvel: 500.0 * Vec2::from_angle(angle),
			angvel: 0.0,
		});
}

pub fn run_boss(
	time: Res<Time>,
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut simulator: ResMut<Simulator>,
	players: Query<&Transform, With<Player>>,
	mut bosses: Query<(&mut Transform, &mut Velocity, &mut EnemyBoss), Without<Player>>,
) {
	let player_pos: Vec2 = players.single().translation.xy();
	for (mut transform, mut velocity, mut boss) in bosses.iter_mut() {
		boss.state = match boss.state {
			BossState::Waiting(ticks) => {
				let t = ticks - time.delta().as_secs_f32();
				if t < 0.0 {
					let dist_at = rand::thread_rng().gen_range::<f32, _>(100.0..300.0);
					let angle_at = rand::thread_rng().gen_range::<f32, _>(0.0..TAU);
					let new_pos = player_pos + Vec2::from_angle(angle_at) * dist_at;
					BossState::Teleporting(new_pos, BOSS_TELEPORT_FRAME, 0)
				} else {
					BossState::Waiting(t)
				}
			},
			BossState::Teleporting(new_pos, ticks, step) => {
				let t = ticks - time.delta().as_secs_f32();
				if t > 0.0 {
					let center_tile = position_to_tile_position(&new_pos);
					for dx in -6..=6 {
						for dy in -6..=6 {
							let x = (center_tile.x as i32 + dx) as usize;
							let y = (center_tile.y as i32 + dy) as usize;
							if dx.abs() + dy.abs() <= step && x < MAP_RADIUS_USIZE * 2 && y < MAP_RADIUS_USIZE * 2 {
								simulator.grid.is_wall[x][y] = false;
							}
						}
					}
					if step >= 7 {
						transform.translation = new_pos.extend(2.0);
						velocity.linvel = Vec2::ZERO;
						let mut rng = rand::thread_rng();
						for _ in 0..5 {
							boss_shoot(&mut commands, &asset_server, new_pos, rng.gen_range(0.0..TAU));
						}
						for _ in 0..3 {
							let a = Vec2::new(1.0, 0.0).angle_between(player_pos - new_pos);
							boss_shoot(
								&mut commands,
								&asset_server,
								new_pos,
								a + rng.gen_range(-TAU / 24.0..TAU / 24.0),
							);
						}
						BossState::ChargingCircle(BOSS_CHARGE_TO_CIRCLE)
					} else {
						BossState::Teleporting(new_pos, BOSS_TELEPORT_FRAME, step + 1)
					}
				} else {
					BossState::Teleporting(new_pos, t, step)
				}
			},
			BossState::ChargingCircle(ticks) => {
				let t = ticks - time.delta().as_secs_f32();
				if t < 0.0 {
					BossState::RunningCircle(CIRCLE_ATTACK_TICK, CIRCLE_ATTACK_TICKS)
				} else {
					BossState::ChargingCircle(t)
				}
			},
			BossState::RunningCircle(ticks, steps) => {
				let t = ticks - time.delta().as_secs_f32();
				let pos = transform.translation.xy();
				if (pos - player_pos).length() > 600.0 {
					BossState::Waiting(BOSS_WAIT_TO_TELEPORT / 2.0)
				} else if t < 0.0 {
					let circle_lines = if steps > 2 * CIRCLE_ATTACK_TICKS / 3 {
						10
					} else if steps > CIRCLE_ATTACK_TICKS / 3 {
						15
					} else {
						18
					};
					for i in 0..16 {
						let a = Vec2::new(1.0, 0.0).angle_between(player_pos - pos);
						boss_shoot(&mut commands, &asset_server, pos, (i as f32) * TAU / (circle_lines as f32));
					}
					if steps == 0 {
						BossState::Waiting(BOSS_WAIT_TO_TELEPORT)
					} else {
						BossState::RunningCircle(CIRCLE_ATTACK_TICK, steps - 1)
					}
				} else {
					BossState::RunningCircle(t, steps)
				}
			},
		}
	}
}
