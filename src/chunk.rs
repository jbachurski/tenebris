use bevy::{math::*, prelude::*, utils::HashSet};
use bevy_ecs_tilemap::prelude::*;

pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 { x: 2, y: 2 };

pub const TILE_SIZE: TilemapTileSize = TilemapTileSize { x: 56.0, y: 51.0 };

#[derive(Debug, Default, Resource)]
pub struct ChunkManager {
	pub spawned_chunks: HashSet<IVec2>,
}

fn chunk_position(position: &Vec2) -> IVec2 {
	(*position
		/ (RENDER_CHUNK_SIZE.as_vec2()
			* Vec2 {
				x: TILE_SIZE.x,
				y: TILE_SIZE.y,
			}))
	.round()
	.as_ivec2()
}

pub fn spawn_chunk(commands: &mut Commands, asset_server: &AssetServer, chunk_position: IVec2) {
	let tilemap_entity = commands.spawn_empty().id();
	let mut tile_storage = TileStorage::empty(RENDER_CHUNK_SIZE.into());
	for x in 0..RENDER_CHUNK_SIZE.x {
		for y in 0..RENDER_CHUNK_SIZE.y {
			let tile_pos = TilePos { x, y };
			let tile_entity = commands
				.spawn(TileBundle {
					position: tile_pos,
					tilemap_id: TilemapId(tilemap_entity),
					..Default::default()
				})
				.id();
			commands.entity(tilemap_entity).add_child(tile_entity);
			tile_storage.set(&tile_pos, tile_entity);
		}
	}

	let transform = Transform::from_translation(Vec3::new(
		chunk_position.x as f32 * RENDER_CHUNK_SIZE.x as f32 * TILE_SIZE.x,
		chunk_position.y as f32 * RENDER_CHUNK_SIZE.y as f32 * TILE_SIZE.y,
		0.0,
	));

	let texture_handle: Handle<Image> = asset_server.load("test.png");
	commands.entity(tilemap_entity).insert(TilemapBundle {
		grid_size: TILE_SIZE.into(),
		size: RENDER_CHUNK_SIZE.into(),
		storage: tile_storage,
		texture: TilemapTexture::Single(texture_handle),
		tile_size: TILE_SIZE,
		transform,
		..Default::default()
	});
}

pub fn spawn_chunks(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	cameras: Query<&Transform, With<Camera>>,
	mut chunk_manager: ResMut<ChunkManager>,
) {
	for camera in cameras.iter() {
		let camera_chunk_position = chunk_position(&camera.translation.xy());
		for x in camera_chunk_position.x - 2..=camera_chunk_position.x + 2 {
			for y in camera_chunk_position.y - 2..=camera_chunk_position.y + 2 {
				let chunk_position = IVec2::new(x, y);
				if !chunk_manager.spawned_chunks.contains(&chunk_position) {
					chunk_manager.spawned_chunks.insert(chunk_position);
					spawn_chunk(&mut commands, &asset_server, chunk_position);
				}
			}
		}
	}
}

pub fn despawn_chunks(
	mut commands: Commands,
	chunks: Query<(Entity, &Transform), With<TilemapSize>>,
	cameras: Query<&Transform, With<Camera>>,
	mut chunk_manager: ResMut<ChunkManager>,
) {
	for camera in cameras.iter() {
		for (entity, transform) in chunks.iter() {
			let position = transform.translation.xy();
			let camera_chunk_position = chunk_position(&camera.translation.xy());
			let chunk_position = chunk_position(&position);
			if chunk_position.x < camera_chunk_position.x - 2
				|| chunk_position.x > camera_chunk_position.x + 2
				|| chunk_position.y < camera_chunk_position.y - 2
				|| chunk_position.y > camera_chunk_position.y + 2
			{
				chunk_manager.spawned_chunks.remove(&chunk_position);
				commands.entity(entity).despawn_recursive();
			}
		}
	}
}
