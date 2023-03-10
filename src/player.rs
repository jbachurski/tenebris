use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{mob::*, shooting::*, utils::MAP_RADIUS};

#[derive(Clone)]
pub enum PlayerWeaponSelect {
	Firebolt,
	Crystals,
	Mine,
}
pub const MAX_HEALTH: i32 = 60;

#[derive(Component)]
pub struct Player {
	pub health: i32,
	pub invincibility_seconds: f32,
	pub gem_count: i32,
	pub select: PlayerWeaponSelect,
	pub level: u32,
}

impl Player {
	pub fn take_damage(self: &mut Self, damage: i32) {
		if self.invincibility_seconds <= 0.0 {
			self.health -= damage;
			self.invincibility_seconds = 0.66;
		}
	}
}

pub fn tick_down_player_invincibility(time: Res<Time>, mut players: Query<(&mut Player)>) {
	for mut player in players.iter_mut() {
		player.invincibility_seconds -= time.delta().as_secs_f32();
	}
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(Timer);

#[derive(Component, Deref, DerefMut)]
pub struct FireboltCooldownTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct CrystalCooldownTimer(pub Timer);

#[derive(Component, Deref, DerefMut)]
pub struct MineCooldownTimer(pub Timer);

#[derive(Default, Resource)]
pub struct MovementPrecedence {
	pub up_has_precedence: Option<bool>,
	pub right_has_precedence: Option<bool>,
}

pub fn update_cooldowns(
	mut timers: Query<(
		&Player,
		&mut FireboltCooldownTimer,
		&mut CrystalCooldownTimer,
		&mut MineCooldownTimer,
	)>,
) {
	for (player, mut fi, mut cr, mut mi) in timers.iter_mut() {
		let a = if player.level == 0 {
			1.0
		} else if player.level == 1 {
			0.8
		} else if player.level == 2 {
			0.6
		} else if player.level == 3 {
			0.4
		} else {
			0.25
		};
		fi.set_duration(Duration::from_secs_f32(a * FIREBALL_COOLDOWN));
		cr.set_duration(Duration::from_secs_f32(a * CRYSTAL_COOLDOWN));
		mi.set_duration(Duration::from_secs_f32(a * MINE_COOLDOWN));
	}
}

pub fn update_level_using_gems(mut players: Query<&mut Player>) {
	for mut player in players.iter_mut() {
		if player.level == 0 && player.gem_count >= 15 {
			player.level = 1;
			player.gem_count -= 15;
		} else if player.level == 1 && player.gem_count >= 30 {
			player.level = 2;
			player.gem_count -= 30;
		} else if player.level == 2 && player.gem_count >= 60 {
			player.level = 3;
			player.gem_count -= 60;
		}
	}
}

pub fn update_select(keyboard_input: Res<Input<KeyCode>>, mut players: Query<&mut Player>) {
	if keyboard_input.just_pressed(KeyCode::Key1) {
		for mut player in players.iter_mut() {
			player.select = PlayerWeaponSelect::Firebolt;
		}
	} else if keyboard_input.just_pressed(KeyCode::Key2) {
		for mut player in players.iter_mut() {
			player.select = PlayerWeaponSelect::Crystals;
		}
	} else if keyboard_input.just_pressed(KeyCode::Key3) {
		for mut player in players.iter_mut() {
			player.select = PlayerWeaponSelect::Mine;
		}
	}
}

pub fn update_velocity(
	mut movement_precedence: Local<MovementPrecedence>,
	keyboard_input: Res<Input<KeyCode>>,
	mut query: Query<(&mut Velocity, &Acceleration), With<Player>>,
) {
	let (mut velocity, acceleration) = query.single_mut();
	let velocity_vec = &mut velocity.linvel;

	let mut acceleration_vec = Vec2::ZERO;
	let mut passive_deceleration = Vec2::ZERO;

	if keyboard_input.just_pressed(KeyCode::A) {
		movement_precedence.right_has_precedence = Some(false);
	} else if keyboard_input.just_released(KeyCode::A) {
		movement_precedence.right_has_precedence = if keyboard_input.pressed(KeyCode::D) {
			Some(true)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::D) {
		movement_precedence.right_has_precedence = Some(true);
	} else if keyboard_input.just_released(KeyCode::D) {
		movement_precedence.right_has_precedence = if keyboard_input.pressed(KeyCode::A) {
			Some(false)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::S) {
		movement_precedence.up_has_precedence = Some(false);
	} else if keyboard_input.just_released(KeyCode::S) {
		movement_precedence.up_has_precedence = if keyboard_input.pressed(KeyCode::W) {
			Some(true)
		} else {
			None
		};
	}
	if keyboard_input.just_pressed(KeyCode::W) {
		movement_precedence.up_has_precedence = Some(true);
	} else if keyboard_input.just_released(KeyCode::W) {
		movement_precedence.up_has_precedence = if keyboard_input.pressed(KeyCode::S) {
			Some(false)
		} else {
			None
		};
	}

	if movement_precedence.right_has_precedence == Some(false) {
		acceleration_vec += Vec2::NEG_X;
		velocity_vec.x = f32::min(0.0, velocity_vec.x);
	} else if movement_precedence.right_has_precedence == Some(true) {
		acceleration_vec += Vec2::X;
		velocity_vec.x = f32::max(0.0, velocity_vec.x);
	} else {
		passive_deceleration.x = -velocity_vec.x;
	}

	if movement_precedence.up_has_precedence == Some(true) {
		acceleration_vec += Vec2::Y;
		velocity_vec.y = f32::max(0.0, velocity_vec.y);
	} else if movement_precedence.up_has_precedence == Some(false) {
		acceleration_vec += Vec2::NEG_Y;
		velocity_vec.y = f32::min(0.0, velocity_vec.y);
	} else {
		passive_deceleration.y = -velocity_vec.y;
	}

	acceleration_vec *= 60.;

	passive_deceleration = passive_deceleration.clamp_length_max(acceleration.rate * 2.0);

	*velocity_vec = (*velocity_vec + acceleration_vec + passive_deceleration).clamp_length_max(acceleration.max_velocity);
}

pub fn animate_player_sprite(
	time: Res<Time>,
	texture_atlases: Res<Assets<TextureAtlas>>,
	mut query: Query<(&mut AnimationTimer, &mut TextureAtlasSprite, &Handle<TextureAtlas>, &Velocity), With<Player>>,
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

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>, mut texture_atlases: ResMut<Assets<TextureAtlas>>) {
	let texture_handle = asset_server.load("wizard.png");
	let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 10, 10, None, None);
	let texture_atlas_handle = texture_atlases.add(texture_atlas);

	commands.spawn((
		Player {
			health: MAX_HEALTH,
			invincibility_seconds: 2.0,
			gem_count: 0,
			select: PlayerWeaponSelect::Firebolt,
			level: 0,
		},
		Velocity::default(),
		Acceleration {
			max_velocity: 5.0 * 60.,
			rate: 2.0 * 60.,
		},
		SpriteSheetBundle {
			texture_atlas: texture_atlas_handle,
			transform: Transform::from_translation(Vec3::new(32. * MAP_RADIUS as f32, 32. * MAP_RADIUS as f32, 2.)),
			..default()
		},
		AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
		Bounded {
			size: Vec2::new(32., 32.),
		},
		CollidesWithWalls,
		FireboltCooldownTimer(Timer::from_seconds(FIREBALL_COOLDOWN, TimerMode::Once)),
		CrystalCooldownTimer(Timer::from_seconds(CRYSTAL_COOLDOWN, TimerMode::Once)),
		MineCooldownTimer(Timer::from_seconds(MINE_COOLDOWN, TimerMode::Once)),
		RigidBody::Dynamic,
		Collider::cuboid(12.0, 12.0),
		Ccd::enabled(),
		LockedAxes::ROTATION_LOCKED,
		SpriteFacingMovement,
	));
}
