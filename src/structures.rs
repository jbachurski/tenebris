use crate::tilesim::Simulator;
use bevy::prelude::*;
use rand::*;

#[derive(Clone, Debug, FromReflect, Reflect)]
pub enum StructureType {
	Unspawned,
	SpawnTutorial,
	Remember,
	BewareSpider,
	Altar,
	BossAltar,
}

pub fn get_structure_texture(structure_type: &StructureType, asset_server: &AssetServer) -> Handle<Image> {
	match *structure_type {
		StructureType::Remember => asset_server.load("grounddeco1.png"),
		StructureType::BewareSpider => asset_server.load("grounddeco2.png"),
		StructureType::Altar => asset_server.load("tutorial.png"), // TODO: Change to altar sprite
		StructureType::SpawnTutorial => asset_server.load("tutorial.png"),
		StructureType::BossAltar => asset_server.load("bossaltar.png"),
		_ => panic!("Tried to get the asset for a structure that does not exist. Is the outer reality bubble too small/big?"),
	}
}

pub fn decide_structure_type(boss_room_loc: UVec2, loc: UVec2) -> StructureType {
	if loc == boss_room_loc {
		return StructureType::BossAltar;
	}
	match thread_rng().gen_range(0..=2) {
		0 => StructureType::Remember,
		1 => StructureType::BewareSpider,
		2 => StructureType::Altar,
		_ => panic!(),
	}
}

pub fn generate_custom_structures(simulator: &mut Simulator) {}
