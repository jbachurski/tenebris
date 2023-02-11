use bevy::{
	math::{Vec2Swizzles, Vec3Swizzles},
	prelude::*,
	render::render_resource::*,
};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use rand::prelude::*;

mod camera;
use camera::*;

mod chunk;
use chunk::*;

mod assets;
use assets::*;

mod tiles;
use tiles::*;

#[derive(Component)]
pub struct Tile;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);

pub const TILE_SIZE: f32 = 32.;

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
		/*.insert_resource(TilemapRenderSettings {
			render_chunk_size: RENDER_CHUNK_SIZE,
		})*/
		//.add_plugin(TilemapPlugin)
		//.insert_resource(ChunkManager::default())
		.insert_resource(Tiles::default())
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.add_plugin(WorldInspectorPlugin)
		.add_startup_system(setup)
		.add_system(update_camera)
		.add_system(spawn_tiles)
		.add_system(despawn_tiles)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut tiles: Res<Tiles>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	// Spawn our camera.
	commands.spawn(Camera2dBundle::default());

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
	//atlases.cave_atlas = asset_server.load("cave/atlas_cave.png");
	atlases.cave_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave.png"),
		Vec2::new(32., 32.),
		51,
		48,
		None,
		None,
	));

	/*
	let block_size = Vec2::splat(TILE_SIZE);
	let center_offset = Vec2::new(-1024.0, 1024.0) / 2.0 + block_size / 2.0 - Vec2::new(0.0, block_size.y);

	let get_block_translation =
		|i: usize, j: usize| center_offset + Vec2::new((j as f32) * block_size.x, -(i as f32) * block_size.y);

	let mut rng = thread_rng();

	for (r, row) in tiles.is_wall.iter().enumerate() {
		for (c, col) in row.iter().enumerate() {
			let id = rng.gen_range(0..(51 * 48));
			commands.spawn(SpriteSheetBundle {
				transform: Transform {
					translation: Vec3::new(r as f32 * TILE_SIZE, c as f32 * TILE_SIZE, 0.),
					scale: Vec2::splat(1.).extend(0.),
					..default()
				},
				sprite: TextureAtlasSprite::new(id),
				texture_atlas: atlases.cave_atlas.clone(),
				..default()
			});
		}
	}*/
}

fn tile_position(position: &Vec2) -> UVec2 {
	(*position / Vec2::splat(TILE_SIZE)).round().as_uvec2()
}

pub fn spawn_tile(commands: &mut Commands, asset_server: &AssetServer, atlases: &Atlases, tiles: &Tiles, tile_position: UVec2) {
	let mut rng = thread_rng();
	let id = rng.gen_range(0..(51 * 48));
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform {
				translation: Vec3::new(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.),
				scale: Vec2::splat(1.).extend(0.),
				..default()
			},
			sprite: TextureAtlasSprite::new/*(id),*/(if tiles.is_wall[tile_position.x as usize][tile_position.y as usize] {0} else {24}),
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
	mut tiles: ResMut<Tiles>,
) {
	for camera in cameras.iter() {
		let camera_tile_position = tile_position(&camera.translation.xy());
		for x in camera_tile_position.x.saturating_sub(8)..=camera_tile_position.x.saturating_add(8) {
			for y in camera_tile_position.y.saturating_sub(8)..=camera_tile_position.y.saturating_add(8) {
				let tile_position = UVec2::new(x, y);
				if !tiles.spawned_tiles.contains(&tile_position) {
					tiles.spawned_tiles.insert(tile_position);
					spawn_tile(&mut commands, &asset_server, &atlases, &tiles, tile_position);
				}
			}
		}
	}
}

pub fn despawn_tiles(
	mut commands: Commands,
	tiles: Query<(Entity, &Transform), With<Tile>>,
	cameras: Query<&Transform, With<Camera>>,
	mut tile_manager: ResMut<Tiles>,
) {
	for camera in cameras.iter() {
		for (entity, transform) in tiles.iter() {
			let position = transform.translation.xy();
			let camera_tile_position = tile_position(&camera.translation.xy());
			let tile_position = tile_position(&position);
			if tile_position.x < camera_tile_position.x.saturating_sub(8)
				|| tile_position.x > camera_tile_position.x.saturating_add(8)
				|| tile_position.y < camera_tile_position.y.saturating_sub(8)
				|| tile_position.y > camera_tile_position.y.saturating_add(8)
			{
				tile_manager.spawned_tiles.remove(&tile_position);
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}
