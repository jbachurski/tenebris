use crate::tilesim::Simulator;
use bevy::prelude::*;

#[derive(Clone, Debug, FromReflect, Reflect)]
pub enum StructureType {
	Unspawned,
	SpawnTutorial,
	Remember,
	BewareSpider,
	Altar,
}

pub fn get_structure_texture(structure_type: &StructureType, asset_server: &AssetServer) -> Handle<Image> {
	match *structure_type {
		StructureType::Remember => asset_server.load("grounddeco1.png"),
		StructureType::BewareSpider => asset_server.load("grounddeco2.png"),
		StructureType::Altar => asset_server.load("tutorial.png"), // TODO: Change to altar sprite
		StructureType::SpawnTutorial => asset_server.load("tutorial.png"),
		_ => panic!("Tried to get the asset for a structure that does not exist. Is the outer reality bubble too small/big?"),
	}
}

pub fn generate_custom_structures(simulator: &mut Simulator) {}
