use std::cmp::min;

use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	math::Vec3Swizzles,
	prelude::*,
	render::render_resource::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::{na::Rotation, prelude::*};

mod camera;
use camera::*;

mod player;
use player::*;

mod enemy;
use enemy::*;

mod assets;
use assets::*;

mod tiles;
use tiles::*;

mod shooting;
use shooting::*;

mod mob;
use mob::*;

mod tilesim;
use tilesim::*;

mod utils;
use utils::*;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);

pub const FOG_RADIUS: u32 = 17;

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
		.add_plugins(
			DefaultPlugins
				.set(AssetPlugin {
					watch_for_changes: true,
					..default()
				})
				.set(WindowPlugin {
					window: WindowDescriptor {
						width: SCREEN_DIMENSIONS.0,
						height: SCREEN_DIMENSIONS.1,
						title: "Tenebris".into(),
						resizable: false,
						mode: WindowMode::Windowed,
						..default()
					},
					..default()
				})
				.set(ImagePlugin {
					default_sampler: SamplerDescriptor {
						mag_filter: FilterMode::Nearest,
						min_filter: FilterMode::Nearest,
						..default()
					},
				}),
		)
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
		.add_plugin(DebugLinesPlugin::default())
		.add_plugin(LogDiagnosticsPlugin::default())
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.insert_resource(Simulator::new(
			MAP_RADIUS * 2,
			(3, 6),
			(10, MAP_RADIUS - 2),
			(10, 13),
			15,
			(20, 30),
			2,
			0,
			20,
			5,
		))
		.insert_resource(SimulatorTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.add_plugin(WorldInspectorPlugin)
		.add_startup_system(setup)
		.add_startup_system(setup_player)
		.add_system(update_velocity)
		.add_system(animate_player_sprite)
		.add_system(player_shoot)
		.add_system(despawn_old_projectiles)
		.add_system(spawn_tiles)
		.add_system(despawn_tiles)
		.add_system(update_tiles)
		.add_system(run_skeleton)
		.add_system(run_wraith)
		.add_system(run_goo)
		//.add_system(move_by_velocity)
		//.add_system(resolve_collisions.before(move_by_velocity))
		.add_system(update_camera) //.after(resolve_collisions))
		.add_system(simulator_step)
		.add_startup_system(spawn_enemies)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut simulator: ResMut<Simulator>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	setup_camera(&mut commands);

	// Spawn a test entity at the origin.
	commands.spawn(SpriteBundle {
		texture: asset_server.load("test.png"),
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
		..default()
	});

	// Create a texture atlas for cave.
	atlases.cave_atlas_simple = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave_simple.png"),
		Vec2::new(32., 32.),
		6,
		4,
		None,
		None,
	));
	atlases.cave_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave.png"),
		Vec2::new(32., 32.),
		51,
		48,
		None,
		None,
	));
	simulator.post_init();
}

fn position_to_tile_position(position: &Vec2) -> UVec2 {
	(*position / Vec2::splat(TILE_SIZE)).round().as_uvec2()
}
fn tile_position_to_position(tile_position: &UVec2) -> Vec2 {
	Vec2::new(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE)
}

#[rustfmt::skip]
fn tile_atlas_index(simulator: &Simulator, tile_position: UVec2) -> usize {
	let f = |dx: i32, dy: i32| -> bool {
		let xx = ((tile_position.x as i32) + dx) as usize;
		let yy = ((tile_position.y as i32) + dy) as usize;
		simulator
			.grid
			.is_wall
			.get(xx)
			.map_or(false, |row| *row.get(yy).unwrap_or(&false))
	};
	let v = ((31*tile_position.x + 37*tile_position.y + 1337) ^ (tile_position.x*7 + tile_position.y*11)) as usize;

	const X: i32 = 1; // wall
	const T: i32 = 0; // any
	const O: i32 = -1; // open

	// pattern center at (1, 1)
	let patterns = [
		(
			[ // up
				[T, O, T],
				[X, X, X],
				[X, X, X],
				[X, X, X]
			],
			613 + v%4
		),
		(
			[ // up left
				[O, O, T],
				[O, X, X],
				[T, X, T],
				[T, T, T]
			],
			2
		),
		(
			[ // up right
				[T, O, O],
				[X, X, O],
				[T, X, T],
				[T, T, T]
			],
			7
		),
		(
			[ // right
				[T, X, T],
				[X, X, O],
				[T, X, T],
				[T, T, T]
			],
			58 + 51 * (v%6)
		),
		(
			[ // left
				[T, X, T],
				[O, X, X],
				[T, X, T],
				[T, T, T]
			],
			53 + 51 * (v%6)
		),
		(
			[ // down low
				[T, X, T],
				[T, O, T],
				[T, T, T],
				[T, T, T]
			],
			562 + v%4
		),
		(
			[ // down mid
				[T, X, T],
				[T, X, T],
				[T, O, T],
				[T, T, T]
			],
			511 + v%4
		),
		(
			[ // down high
				[T, X, T],
				[T, X, T],
				[T, X, T],
				[T, O, T]
			],
			460 + v%4
		),
		(
			[ // mid
				[X, X, X],
				[X, X, X],
				[X, X, X],
				[T, T, T]
			],
			54 + v%4 + 51*((v/4)%5)
		),
		(
			[ // generic open
				[T, T, T],
				[T, O, T],
				[T, T, T],
				[T, T, T]
			],
			1775 + v%3 + 51*((v/3)%3)
		),
		(
			[ // generic wall
				[T, T, T],
				[T, X, T],
				[T, T, T],
				[T, T, T]
			],
			208
		)
	];

	for (pattern, target) in patterns {
		let mut ok = true;
		for (dy, row) in (-1..=2).zip(pattern) {
			for (dx, a) in (-1..=1).zip(row) {
				if !((f(dx, -dy) && a >= 0) || (!f(dx, -dy) && a <= 0)) {
					ok = false;
					break;
				}
			}
		}
		if ok { return target; }
	};
	panic!();

	/*
	if !f(0, 0) && f(0, 1) && !f(-1, 0) && f(1, 1) {
		410 // lower left wall bottom
	} else if f(0, 0) && !f(0, -1) && !f(-1, 0) && f(0, 1) && f(1, 0) {
		359 // lower left wall middle
	} else if f(0, 0) && !f(0, -2) && f(0, -1) && !f(-1, 0) && f(1, 0) {
		308 // lower left wall top
	} else if !f(0, 0) && f(0, 1) {
		562 + v%4  // up wall bottom, at floor, 0..4 variants
	} else if f(0, 0) && !f(0, -1) {
		511 + v%4  // up wall middle, above floor, 0..4 variants
	} else if f(0, 0) && f(0, -1) && !f(0, -2) {
		460 + v%4  // up wall top, 0..4 variants
	} else if f(0, 0) && !f(0, 1) {
		613 + v%4  // bot wall, 0..4 variants
	} else if f(0, 0) && !f(1, 0) && f(-1, 0) {
		58 + 51*(v%6) // wall right, next to floor on left, 0..6 variants
	} else if f(0, 0) && !f(-1, 0) && f(1, 0) {
		53 + 51*(v%6) // wall left, next to floor on right, 0..6 variants
	} else if f(0, 0) {
		208  // generic
	} else {
		1775 + v%3 + 51*((v/3)%3) // empty
	}
	*/
	// if f(0, 0) {
	// 	if f(0, -1) {
	// 		208
	// 	} else {
	// 		if !f(0, 1) {
	// 			613
	// 		} else {
	// 			208
	// 		}
	// 	}
	// } else {
	// 	1777
	// }
}

pub fn spawn_tile(
	commands: &mut Commands,
	_asset_server: &AssetServer,
	atlases: &Atlases,
	simulator: &Simulator,
	tile_position: UVec2,
) {
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.),
			sprite: TextureAtlasSprite::new(tile_atlas_index(simulator, tile_position)),
			texture_atlas: atlases.cave_atlas.clone(),
			..default()
		})
		.insert(RigidBody::Fixed)
		.insert(Velocity::default())
		.insert(Sensor)
		.insert(Tile);
}

pub fn spawn_tiles(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	atlases: Res<Atlases>,
	cameras: Query<&Transform, With<Camera>>,
	mut simulator: ResMut<Simulator>,
) {
	for camera in cameras.iter() {
		let camera_tile_position = position_to_tile_position(&camera.translation.xy());
		for x in camera_tile_position.x.saturating_sub(FOG_RADIUS)..=min(199, camera_tile_position.x.saturating_add(FOG_RADIUS))
		{
			for y in
				camera_tile_position.y.saturating_sub(FOG_RADIUS)..=min(199, camera_tile_position.y.saturating_add(FOG_RADIUS))
			{
				let tile_position = UVec2::new(x, y);
				if !simulator.grid.spawned_tiles.contains(&tile_position) {
					simulator.grid.spawned_tiles.insert(tile_position);
					spawn_tile(&mut commands, &asset_server, &atlases, &simulator, tile_position);
				}
			}
		}
	}
}

pub fn simulator_step(
	mut simulator: ResMut<Simulator>,
	mut player: Query<&Transform, With<Player>>,
	mut timer: ResMut<SimulatorTimer>,
	time: Res<Time>,
) {
	timer.0.tick(time.delta());
	if timer.0.just_finished() {
		let player_trans = player.single().translation.truncate();
		let player_pos = position_to_tile_position(&player_trans);
		simulator.step(player_pos);
	}
}

pub fn despawn_tiles(
	mut commands: Commands,
	tiles: Query<(Entity, &Transform), With<Tile>>,
	cameras: Query<&Transform, With<Camera>>,
	mut simulator: ResMut<Simulator>,
) {
	for camera in cameras.iter() {
		for (entity, transform) in tiles.iter() {
			let position = transform.translation.xy();
			let camera_tile_position = position_to_tile_position(&camera.translation.xy());
			let tile_position = position_to_tile_position(&position);
			if tile_position.x < camera_tile_position.x.saturating_sub(FOG_RADIUS)
				|| tile_position.x > camera_tile_position.x.saturating_add(FOG_RADIUS)
				|| tile_position.y < camera_tile_position.y.saturating_sub(FOG_RADIUS)
				|| tile_position.y > camera_tile_position.y.saturating_add(FOG_RADIUS)
			{
				simulator.grid.spawned_tiles.remove(&tile_position);
				commands.entity(entity).despawn();
			}
		}
	}
}

pub fn update_tiles(
	mut commands: Commands,
	mut tiles: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Tile>>,
	simulator: ResMut<Simulator>,
) {
	for (entity, transform, mut ta_sprite) in tiles.iter_mut() {
		let tile_position = position_to_tile_position(&transform.translation.xy());
		if simulator.grid.spawned_tiles.contains(&tile_position) {
			*ta_sprite = TextureAtlasSprite::new(tile_atlas_index(simulator, tile_position));
		}
	}
}
