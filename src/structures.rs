use bevy::prelude::*;

#[derive(Clone, Debug, FromReflect, Reflect)]
pub enum StructureType {
	Unspawned,
	Remember,
	BewareSpider,
	Altar,
}

pub fn get_structure_texture(structure_type: &StructureType, asset_server: &AssetServer) -> Handle<Image> {
	match *structure_type {
		StructureType::Remember => asset_server.load("grounddeco1.png"),
		StructureType::BewareSpider => asset_server.load("grounddeco2.png"),
		StructureType::Altar => asset_server.load("tutorial.png"), // TODO: Change to altar sprite
		_ => panic!(),
	}
}
