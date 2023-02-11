use bevy::{prelude::*, render::render_resource::*};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

mod camera;
use camera::*;

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
        .add_plugin(WorldInspectorPlugin)
        .add_startup_system(setup)
        .add_system(update_camera)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: asset_server.load("test.png"),
        ..default()
    });
}