use bevy::{math::*, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 8, y: 8 };

pub const TILE_SIZE: UVec2 = UVec2 { x: 8, y: 8 };

#[derive(Debug, Default, Resource)]
pub struct ChunkManager {
	pub spawned_chunks: HashSet<IVec2>,
}

fn chunk_position(position: &Vec2) -> IVec2 {
	position.as_ivec2() / (RENDER_CHUNK_SIZE * TILE_SIZE).as_ivec2()
}

pub fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_position: IVec2) {
	let tilemap_entity = commands.spawn_empty().id();
	let mut tile_storage = TileStorage::empty(RENDER_CHUNK_SIZE.into());
	for x in 0..RENDER_CHUNK_SIZE.x {
		for y in 0..RENDER_CHUNK_SIZE.y {}
	}
}

pub fn spawn_chunks(mut commands: Commands, asset_server: Res<AssetServer>, cameras: Query<&Transform, With<Camera>>, mut chunk_manager: ResMut<ChunkManager>) {
	for camera in cameras.iter() {
		let camera_chunk_position = chunk_position(&camera.translation.xy());
		for x in camera_chunk_position.x - 2..camera_chunk_position.x + 2 {
			for y in camera_chunk_position.y - 2..camera_chunk_position.y + 2 {
				let chunk_position = IVec2::new(x, y);
				if chunk_manager.spawned_chunks.contains(&chunk_position) {
					chunk_manager.spawned_chunks.insert(chunk_position);
				}
			}
		}
	}
}

pub fn despawn_chunks() {}
