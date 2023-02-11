use bevy::{prelude::*, utils::HashSet};

#[derive(Debug, Resource)]
pub struct Tiles {
	pub is_wall: [[bool; 200]; 200],
	pub spawned_tiles: HashSet<UVec2>,
}

impl Default for Tiles {
	fn default() -> Self {
		return Self {
			is_wall: [[false; 200]; 200],
			spawned_tiles: default(),
		};
	}
}
