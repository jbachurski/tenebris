use bevy::prelude::*;

use crate::tilesim::Simulator;

#[derive(Component)]
pub struct Minimap;

pub fn get_minimap_color(simulator: &Simulator, i: u32, j: u32) -> BackgroundColor {
	if simulator.grid.is_wall[i as usize][j as usize] {
		return Color::rgba(0.4, 0.4, 1.0, 0.2).into();
	}
	return Color::rgba(0.4, 0.4, 1.0, 0.5).into();
}
