use bevy::{prelude::*, render::render_resource::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_ecs_tilemap::prelude::*;

mod camera;
use camera::*;

mod chunk;
use chunk::*;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);
pub const RENDER_CHUNK_SIZE: UVec2 = UVec2 {
    x: 8,
    y: 8,
};

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
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup)
        .add_system(update_camera)
        .add_system(spawn_chunks)
        .add_system(despawn_chunks)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("test.png"),
        ..default()
    });
}