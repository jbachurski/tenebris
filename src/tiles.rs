use bevy::{prelude::*, render::extract_resource::ExtractResource, utils::HashSet};
use bevy_inspector_egui::prelude::*;
use rand::random;

use crate::utils::*;

pub const TILE_SIZE: f32 = 32.;

#[derive(Component)]
pub struct Tile;

pub fn position_to_tile_position(position: &Vec2) -> UVec2 {
	(*position / Vec2::splat(TILE_SIZE)).round().as_uvec2()
}

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
