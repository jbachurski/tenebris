use bevy::{prelude::*, sprite::TextureAtlas};

#[derive(Default, Resource)]
pub struct Atlases {
	pub cave_atlas_simple: Handle<TextureAtlas>,
	pub cave_atlas: Handle<TextureAtlas>,
	pub campfire_atlas: Handle<TextureAtlas>,
}
