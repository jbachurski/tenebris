use std::cmp::min;

use bevy::{
	math::Vec3Swizzles,
	prelude::*,
	render::{extract_resource::ExtractResource, view::RenderLayers},
	utils::*,
};
use bevy_inspector_egui::prelude::*;
use bevy_rapier2d::{parry::query::details::CompositeShapeAgainstAnyDistanceVisitor, prelude::*};

use crate::{assets::Atlases, player::Player, structures::*, tilemap::*, tilesim::Simulator, utils::*, Despawn};

pub const TILE_SIZE: f32 = 32.;
pub const FOG_RADIUS: u32 = 17;

#[derive(Component)]
pub struct Tile;

#[derive(Component)]
pub struct BackTile;

#[derive(Component)]
pub struct Structure;

#[derive(Component)]
pub struct Overlay;

#[derive(Clone, Debug, Resource, InspectorOptions, ExtractResource)]
pub struct TileManager {
	pub is_wall: Box<[[bool; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2]>,
	pub lightmap: Box<[[f32; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2]>,
	pub spawned_tiles: HashSet<UVec2>,
	pub campfires: HashSet<UVec2>,
	pub structures: HashMap<UVec2, StructureType>,
	pub reality_bubble: HashSet<UVec2>,
}

impl Default for TileManager {
	fn default() -> Self {
		return Self {
			is_wall: Box::new([[false; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2]),
			lightmap: Box::new([[0.; MAP_RADIUS_USIZE * 2]; MAP_RADIUS_USIZE * 2]),
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
pub fn _tile_position_to_position(tile_position: &UVec2) -> Vec2 {
	Vec2::new(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE)
}
pub fn spawn_tile(
	commands: &mut Commands,
	asset_server: &AssetServer,
	atlases: &Atlases,
	simulator: &Simulator,
	tile_position: UVec2,
) {
	let v = tile_position_rand(tile_position);
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.),
			sprite: TextureAtlasSprite::new(1775 + v % 3 + 51 * ((v / 3) % 3)),
			texture_atlas: atlases.cave_atlas.clone(),
			..default()
		})
		.insert(BackTile);

	commands
		.spawn(SpriteSheetBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.5),
			sprite: TextureAtlasSprite::new(tile_atlas_index(simulator, tile_position)),
			texture_atlas: atlases.cave_atlas.clone(),
			..default()
		})
		.insert(RigidBody::Fixed)
		.insert(Velocity::default())
		.insert(Collider::cuboid(16., 16.))
		.insert(Sensor)
		.insert(Tile);

	// Check if campfire tile
	if simulator.grid.campfires.contains(&tile_position) {
		spawn_campfire_sprite(commands, atlases, tile_position);
	}

	// Check if other structure tile
	simulator.grid.structures.get(&tile_position).map(|structure_type| {
		spawn_structure_sprite(commands, asset_server, structure_type, tile_position);
	});
}

pub fn spawn_structure_sprite(
	commands: &mut Commands,
	asset_server: &AssetServer,
	structure_type: &StructureType,
	tile_position: UVec2,
) {
	commands
		.spawn(SpriteBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.6),
			texture: get_structure_texture(structure_type, asset_server),
			..default()
		})
		.insert(Structure);
}

pub fn spawn_campfire_sprite(commands: &mut Commands, atlases: &Atlases, tile_position: UVec2) {
	commands
		.spawn(SpriteSheetBundle {
			transform: Transform::from_xyz(tile_position.x as f32 * TILE_SIZE, tile_position.y as f32 * TILE_SIZE, 0.6),
			sprite: TextureAtlasSprite::new(0),
			texture_atlas: atlases.campfire_atlas.clone(),
			..default()
		})
		.insert(Structure);
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
		for x in camera_tile_position.x.saturating_sub(FOG_RADIUS)
			..=min(MAP_RADIUS * 2 - 1, camera_tile_position.x.saturating_add(FOG_RADIUS))
		{
			for y in camera_tile_position.y.saturating_sub(FOG_RADIUS)
				..=min(MAP_RADIUS * 2 - 1, camera_tile_position.y.saturating_add(FOG_RADIUS))
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
	back_tiles: Query<(Entity, &Transform), With<BackTile>>,
	structures: Query<(Entity, &Transform), With<Structure>>,
	overlays: Query<(Entity, &Transform), With<Overlay>>,
	cameras: Query<&Transform, With<Camera>>,
	mut simulator: ResMut<Simulator>,
) {
	for camera in cameras.iter() {
		for (entity, transform) in tiles
			.iter()
			.chain(back_tiles.iter())
			.chain(structures.iter())
			.chain(overlays.iter())
		{
			let position = transform.translation.xy();
			let camera_tile_position = position_to_tile_position(&camera.translation.xy());
			let tile_position = position_to_tile_position(&position);
			if tile_position.x < camera_tile_position.x.saturating_sub(FOG_RADIUS)
				|| tile_position.x > camera_tile_position.x.saturating_add(FOG_RADIUS)
				|| tile_position.y < camera_tile_position.y.saturating_sub(FOG_RADIUS)
				|| tile_position.y > camera_tile_position.y.saturating_add(FOG_RADIUS)
			{
				simulator.grid.spawned_tiles.remove(&tile_position);
				commands.entity(entity).insert(Despawn);
			}
		}
	}
}

pub fn update_lightmap(
	mut commands: Commands,
	mut overlays: Query<(Entity, &Transform), With<Overlay>>,
	mut simulator: ResMut<Simulator>,
	mut player: Query<&Transform, With<Player>>,
	cameras: Query<&Transform, With<Camera>>,
) {
	let player_pos = player.single().translation.truncate();
	simulator.recalc_lightmap(position_to_tile_position(&player_pos));
	for (entity, transform) in overlays.iter_mut() {
		let tile_position = position_to_tile_position(&transform.translation.xy());
		let (i, j) = tile_position.into();
		commands.entity(entity).insert(Despawn);
	}

	for camera in cameras.iter() {
		let camera_tile_position = position_to_tile_position(&camera.translation.xy());
		for x in camera_tile_position.x.saturating_sub(FOG_RADIUS)
			..=min(MAP_RADIUS * 2 - 1, camera_tile_position.x.saturating_add(FOG_RADIUS))
		{
			for y in camera_tile_position.y.saturating_sub(FOG_RADIUS)
				..=min(MAP_RADIUS * 2 - 1, camera_tile_position.y.saturating_add(FOG_RADIUS))
			{
				let tile_position = UVec2::new(x, y);
				commands
					.spawn(SpriteBundle {
						transform: Transform::from_xyz(
							tile_position.x as f32 * TILE_SIZE,
							tile_position.y as f32 * TILE_SIZE,
							10.,
						),
						sprite: Sprite {
							color: Color::rgba(0., 0., 0., 1. - simulator.grid.lightmap[x as usize][y as usize]),
							custom_size: Some(Vec2::new(TILE_SIZE, TILE_SIZE)),
							..default()
						},
						..default()
					})
					.insert(Overlay);
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
			*ta_sprite = TextureAtlasSprite::new(tile_atlas_index(&simulator, tile_position));
			if simulator.grid.is_wall[tile_position.x as usize][tile_position.y as usize] {
				commands.entity(entity).remove::<Sensor>();
			} else {
				commands.entity(entity).insert(Sensor);
			}
		}
	}
}
