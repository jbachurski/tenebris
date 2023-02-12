use std::cmp::min;

use bevy::{math::Vec3Swizzles, prelude::*, render::extract_resource::ExtractResource, utils::HashSet};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::{assets::Atlases, tilemap::*, tilesim::Simulator, utils::*};

pub const TILE_SIZE: f32 = 32.;
pub const FOG_RADIUS: u32 = 17;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct BackTile;

#[derive(Reflect, Clone, Debug, Resource, InspectorOptions, ExtractResource)]
#[reflect(Resource, InspectorOptions)]
pub struct TileManager {
	pub is_wall: [[bool; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2],
	pub spawned_tiles: HashSet<UVec2>,
	pub campfires: HashSet<UVec2>,
	pub structures: HashSet<UVec2>,
	pub reality_bubble: HashSet<UVec2>,
}

impl Default for TileManager {
	fn default() -> Self {
		return Self {
			is_wall: [[false; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2],
			spawned_tiles: default(),
			campfires: default(),
			structures: default(),
			reality_bubble: default(),
		};
	}
}

pub fn position_to_tile_position(position: &Vec2) -> UVec2 {
	(*position / Vec2::splat(TILE_SIZE)).round().as_uvec2()
}
fn _tile_position_to_position(tile_position: &UVec2) -> Vec2 {
	Vec2::new(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE)
}
pub fn spawn_tile(
	commands: &mut Commands,
	_asset_server: &AssetServer,
	atlases: &Atlases,
	simulator: &Simulator,
	tile_position: UVec2,
) {
	let v = tile_position_rand(tile_position);
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
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.),
			sprite: TextureAtlasSprite::new(1775 + v % 3 + 51 * ((v / 3) % 3)),
			texture_atlas: atlases.cave_atlas.clone(),
			..default()
		})
		.insert(BackTile);
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

pub fn update_tiles(mut tiles: Query<(Entity, &Transform, &mut TextureAtlasSprite), With<Tile>>, simulator: ResMut<Simulator>) {
	for (_entity, transform, mut ta_sprite) in tiles.iter_mut() {
		let tile_position = position_to_tile_position(&transform.translation.xy());
		if simulator.grid.spawned_tiles.contains(&tile_position) {
			*ta_sprite = TextureAtlasSprite::new(tile_atlas_index(&simulator, tile_position));
		}
	}
}
