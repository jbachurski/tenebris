use std::cmp::min;

use bevy::{
	math::{Vec2Swizzles, Vec3Swizzles},
	prelude::*,
	render::render_resource::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use rand::prelude::*;

mod camera;
use camera::*;

mod chunk;
use chunk::*;

mod player;
use player::*;

mod enemy;
use enemy::*;

mod assets;
use assets::*;

mod tiles;
use tiles::*;

#[derive(Component)]
pub struct Tile;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);

pub const TILE_SIZE: f32 = 32.;

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
		.add_plugin(DebugLinesPlugin::default())
		.insert_resource(TileManager::default())
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.add_plugin(WorldInspectorPlugin)
		.add_startup_system(setup)
		.add_startup_system(setup_player)
		.add_system(update_velocity)
		.add_system(move_player)
		.add_system(animate_player_sprite)
		.add_system(update_camera.after(move_player))
		.add_system(spawn_tiles)
		.add_system(despawn_tiles)
		.add_system(update_tiles)
		.add_system(run_skeleton)
		.add_system(run_wraith)
		.add_startup_system(spawn_enemies)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut tiles: Res<TileManager>,
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
	atlases.cave_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave.png"),
		Vec2::new(32., 32.),
		51,
		48,
		None,
		None,
	));
}

fn position_to_tile_position(position: &Vec2) -> UVec2 {
	(*position / Vec2::splat(TILE_SIZE)).round().as_uvec2()
}

pub fn spawn_tile(
	commands: &mut Commands,
	asset_server: &AssetServer,
	atlases: &Atlases,
	tile_manager: &TileManager,
	tile_position: UVec2,
) {
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform {
				translation: Vec3::new(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.),
				scale: Vec2::splat(1.).extend(0.),
				..default()
			},
			sprite: TextureAtlasSprite::new(if tile_manager.is_wall[tile_position.x as usize][tile_position.y as usize] {
				0
			} else {
				1460
			}),
			texture_atlas: atlases.cave_atlas.clone(),
			..default()
		})
		.insert(Tile);
}

pub fn spawn_tiles(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	atlases: Res<Atlases>,
	cameras: Query<&Transform, With<Camera>>,
	mut tile_manager: ResMut<TileManager>,
) {
	for camera in cameras.iter() {
		let camera_tile_position = position_to_tile_position(&camera.translation.xy());
		for x in camera_tile_position.x.saturating_sub(FOG_RADIUS)..=min(199, camera_tile_position.x.saturating_add(FOG_RADIUS))
		{
			for y in
				camera_tile_position.y.saturating_sub(FOG_RADIUS)..=min(199, camera_tile_position.y.saturating_add(FOG_RADIUS))
			{
				let tile_position = UVec2::new(x, y);
				if !tile_manager.spawned_tiles.contains(&tile_position) {
					tile_manager.spawned_tiles.insert(tile_position);
					spawn_tile(&mut commands, &asset_server, &atlases, &tile_manager, tile_position);
				}
			}
		}
	}
}

pub fn despawn_tiles(
	mut commands: Commands,
	tiles: Query<(Entity, &Transform), With<Tile>>,
	cameras: Query<&Transform, With<Camera>>,
	mut tile_manager: ResMut<TileManager>,
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
				tile_manager.spawned_tiles.remove(&tile_position);
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}

pub fn update_tiles(
	mut tiles: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Tile>>,
	tile_manager: ResMut<TileManager>,
) {
	for (entity, transform, mut ta_sprite) in tiles.iter_mut() {
		let tile_position = position_to_tile_position(&transform.translation.xy());
		if tile_manager.spawned_tiles.contains(&tile_position) {
			*ta_sprite = TextureAtlasSprite::new(if tile_manager.is_wall[tile_position.x as usize][tile_position.y as usize] {
				0
			} else {
				1460
			});
		}
	}
}
