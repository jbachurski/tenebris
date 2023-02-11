use bevy::{prelude::*, render::extract_resource::ExtractResource, utils::HashSet};
use bevy_inspector_egui::prelude::*;
use rand::random;

#[derive(Reflect, Clone, Debug, Resource, InspectorOptions, ExtractResource)]
#[reflect(Resource, InspectorOptions)]
pub struct TileManager {
	pub is_wall: [[bool; 200]; 200],
	pub spawned_tiles: HashSet<UVec2>,
	pub campfires: HashSet<UVec2>,
	pub structures: HashSet<UVec2>,
	pub reality_bubble: HashSet<UVec2>,
}

impl Default for TileManager {
	fn default() -> Self {
		let mut is_wall = [[false; 200]; 200];
		for i in 1..199 {
			for j in 1..199 {
				is_wall[i][j] = random();
			}
		}
		let mut any: bool = true;
		while any {
			any = false;
			let mut next_wall = [[false; 200]; 200];
			for i in 1..199 {
				for j in 1..199 {
					let c: u32 = [
						is_wall[i - 1][j - 1],
						is_wall[i - 1][j],
						is_wall[i - 1][j + 1],
						is_wall[i][j - 1],
						is_wall[i][j + 1],
						is_wall[i + 1][j - 1],
						is_wall[i + 1][j],
						is_wall[i + 1][j + 1],
					]
					.iter()
					.map(|x| *x as u32)
					.sum();
					next_wall[i][j] = if c <= 3 {
						false
					} else if c >= 6 {
						true
					} else {
						is_wall[i][j]
					};
					if is_wall[i][j] != next_wall[i][j] {
						any = true;
					}
				}
			}
			for i in 0..200 {
				for j in 0..200 {
					is_wall[i][j] = next_wall[i][j]
				}
			}
		}
		return Self {
			is_wall,
			spawned_tiles: default(),
			campfires: default(),
			structures: default(),
			reality_bubble: default(),
		};
	}
}
