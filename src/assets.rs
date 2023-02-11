use bevy::{prelude::*, sprite::TextureAtlas};

#[derive(Default, Resource)]
pub struct Atlases {
	pub cave_atlas: Handle<TextureAtlas>,
}
