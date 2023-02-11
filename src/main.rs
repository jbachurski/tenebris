use bevy::{math::Vec2Swizzles, prelude::*, render::render_resource::*};
use bevy_ecs_tilemap::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
use camera::*;

mod chunk;
use chunk::*;

mod assets;
use assets::*;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);

fn main() {
	App::new()
		.insert_resource(ClearColor(Color::rgb_u8(0, 0, 0)))
		.add_plugins(
			DefaultPlugins
				.set(AssetPlugin {
					watch_for_changes: true,
					..default()
				})
				.set(WindowPlugin {
					window: WindowDescriptor {
						width: SCREEN_DIMENSIONS.0,
						height: SCREEN_DIMENSIONS.1,
						title: "Tenebris".into(),
						resizable: false,
						mode: WindowMode::Windowed,
						..default()
					},
					..default()
				})
				.set(ImagePlugin {
					default_sampler: SamplerDescriptor {
						mag_filter: FilterMode::Nearest,
						min_filter: FilterMode::Nearest,
						..default()
					},
				}),
		)
		.insert_resource(TilemapRenderSettings {
			render_chunk_size: RENDER_CHUNK_SIZE,
		})
		.add_plugin(TilemapPlugin)
		.insert_resource(ChunkManager::default())
		.insert_resource(Atlases::default())
		.add_plugin(WorldInspectorPlugin)
		.add_startup_system(setup)
		.add_system(update_camera)
		.add_system(spawn_chunks)
		.add_system(despawn_chunks)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	// Spawn our camera.
	commands.spawn(Camera2dBundle::default());

	// Spawn a test entity at the origin.
	commands.spawn(SpriteBundle {
		texture: asset_server.load("test.png"),
		transform: Transform {
			translation: Vec3::new(0.0, 0.0, 2.0),
			..Default::default()
		},
		..default()
	});

	// Create a texture atlas for cave.
	atlases.cave_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave.png"),
		Vec2::new(32., 32.),
		51,
		48,
		None,
		None,
	));
}
