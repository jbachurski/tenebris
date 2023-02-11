use bevy::{prelude::*, render::extract_resource::ExtractResource, utils::HashSet};
use bevy_inspector_egui::prelude::*;

#[derive(Reflect, Clone, Debug, Resource, InspectorOptions, ExtractResource)]
#[reflect(Resource, InspectorOptions)]
pub struct TileManager {
	pub is_wall: [[bool; 200]; 200],
	pub spawned_tiles: HashSet<UVec2>,
}

impl Default for TileManager {
	fn default() -> Self {
		return Self {
			is_wall: [[false; 200]; 200],
			spawned_tiles: default(),
		};
	}
}
