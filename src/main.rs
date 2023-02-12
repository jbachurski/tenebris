use bevy::{
	diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	prelude::*,
	render::render_resource::*,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

mod camera;
use camera::*;

mod player;
use player::*;

mod enemy;
use enemy::*;

mod assets;
use assets::*;

mod tiles;
use tiles::*;

mod shooting;
use shooting::*;

mod minimap;
use minimap::*;

mod mob;

mod tilemap;

mod tilesim;
use tilesim::*;

mod utils;
use utils::*;

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
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
		.add_plugin(DebugLinesPlugin::default())
		.add_plugin(LogDiagnosticsPlugin::default())
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.insert_resource(Simulator::new(
			MAP_RADIUS * 2,
			(3, 6),
			(10, MAP_RADIUS - 2),
			(10, 13),
			15,
			(20, 30),
			2,
			0,
			20,
			5,
		))
		.insert_resource(SimulatorTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.add_plugin(WorldInspectorPlugin)
		.add_startup_system(setup)
		.add_startup_system(setup_player)
		.add_system(update_velocity)
		.add_system(animate_player_sprite)
		.add_system(player_shoot)
		.add_system(despawn_old_projectiles)
		.add_system(spawn_tiles)
		.add_system(despawn_tiles)
		.add_system(update_tiles)
		.add_system(run_skeleton)
		.add_system(run_wraith)
		.add_system(run_goo)
		//.add_system(move_by_velocity)
		//.add_system(resolve_collisions.before(move_by_velocity))
		.add_system(update_camera) //.after(move_by_velocity))
		.add_system(simulator_step)
		.add_startup_system(spawn_enemies)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut simulator: ResMut<Simulator>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
	setup_camera(&mut commands);

	// Spawn a UI
	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
				justify_content: JustifyContent::SpaceBetween,
				..default()
			},
			..default()
		})
		.with_children(|parent| {
			parent
				.spawn(NodeBundle {
					style: Style {
						size: Size::new(Val::Px(MINIMAP_SIZE), Val::Px(MINIMAP_SIZE)),
						position_type: PositionType::Absolute,
						position: UiRect {
							right: Val::Px(10.0),
							top: Val::Px(10.0),
							..default()
						},
						//border: UiRect::all(Val::Px(20.0)),
						..default()
					},
					background_color: Color::rgba(0.4, 0.4, 1.0, 0.1).into(),
					..default()
				})
				.insert(Minimap);
		});

	// Create a texture atlas for cave.
	atlases.cave_atlas_simple = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave_simple.png"),
		Vec2::new(32., 32.),
		6,
		4,
		None,
		None,
	));
	atlases.cave_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("cave/atlas_cave.png"),
		Vec2::new(32., 32.),
		51,
		48,
		None,
		None,
	));
	simulator.post_init();
}

pub fn simulator_step(
	mut commands: Commands,
	mut simulator: ResMut<Simulator>,
	player: Query<&Transform, With<Player>>,
	minimap: Query<Entity, With<Minimap>>,
	mut timer: ResMut<SimulatorTimer>,
	time: Res<Time>,
) {
	timer.0.tick(time.delta());
	if timer.0.just_finished() {
		let player_trans = player.single().translation.truncate();
		let player_pos = position_to_tile_position(&player_trans);
		simulator.step(player_pos);

		// Process minimap
		let minimap_entity = minimap.single();
		commands.entity(minimap_entity).despawn_descendants();
		commands.entity(minimap_entity).insert(Minimap).with_children(|parent| {
			let elem_width = MINIMAP_SIZE / (2.0 * MAP_RADIUS as f32);
			for i in 0..MAP_RADIUS * 2 {
				for j in 0..MAP_RADIUS * 2 {
					let loc = UVec2::new(i, j);
					if simulator.grid.reality_bubble.contains(&loc) {
						parent.spawn(NodeBundle {
							style: Style {
								size: Size::new(Val::Px(elem_width), Val::Px(elem_width)),
								position_type: PositionType::Absolute,
								position: UiRect {
									left: Val::Px(elem_width * i as f32),
									bottom: Val::Px(elem_width * j as f32),
									..default()
								},
								..default()
							},
							background_color: get_minimap_color(&simulator, i, j),
							..default()
						});
					}
				}
			}
		});
	}
}
