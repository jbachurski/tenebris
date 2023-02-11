use bevy::{prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

#[derive(Debug, Default, Resource)]
pub struct ChunkManager {
	pub spawned_chunks: HashSet<IVec2>,
}

pub fn spawn_chunks(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	camera_query: Query<&Transform, With<Camera>>,
	mut chunk_manager: ResMut<ChunkManager>,
) {

}

pub fn despawn_chunks() {

}