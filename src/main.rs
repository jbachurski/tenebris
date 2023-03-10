use bevy::{
	diagnostic::{EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
	prelude::*,
	render::render_resource::*,
};
use bevy_prototype_debug_lines::*;
use bevy_rapier2d::prelude::*;

mod camera;
use camera::*;

mod player;
use player::*;

mod enemies;
use enemies::{boss::*, goo::*, ranger::*, wraith::*, *};

mod assets;
use assets::*;

mod tiles;
use tiles::*;

mod shooting;
use shooting::*;

mod hud;
use hud::*;

mod mob;
use mob::*;

mod tilemap;

mod tilesim;
use tilesim::*;

mod utils;
use utils::*;

mod gems;
use gems::*;

mod structures;

#[derive(Component)]
pub struct Despawn;

pub const SCREEN_DIMENSIONS: (f32, f32) = (1024.0, 768.0);

pub const DESPAWN_STAGE: &str = "DESPAWN";
const TIME_STEP: f32 = 1.0 / 60.0;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
	Alive,
	Dead,
}

fn main() {
	App::new()
		.add_state(AppState::Alive)
		.insert_resource(ClearColor(Color::rgb_u8(1, 0, 0)))
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.insert_resource(SimulatorTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
		.insert_resource(EnemySpawner {
			timer: Timer::from_seconds(1.0, TimerMode::Repeating),
			max_enemy_count: 4,
		})
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
						title: "Memorynth".into(),
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
		.add_system_set(SystemSet::on_update(AppState::Alive).with_system(check_if_dead))
		.add_system_set(SystemSet::on_update(AppState::Dead).with_system(enter_dead))
		.add_system_set(SystemSet::on_enter(AppState::Alive).with_system(reset_vars))
		.add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(32.))
		.add_plugin(DebugLinesPlugin::default())
		.add_plugin(LogDiagnosticsPlugin::default())
		.add_plugin(FrameTimeDiagnosticsPlugin::default())
		.add_plugin(EntityCountDiagnosticsPlugin::default())
		.insert_resource(Simulator::new(
			MAP_RADIUS * 2,
			(3, 6),
			(10, MAP_RADIUS - 6),
			(10, 13),
			15,
			(20, 30),
			10,
			10,
			20,
			5,
			20,
		))
		.insert_resource(SimulatorTimer(Timer::from_seconds(0.1, TimerMode::Repeating)))
		.insert_resource(Atlases::default())
		.insert_resource(Msaa { samples: 1 })
		.insert_resource(EnemySpawner {
			timer: Timer::from_seconds(1.0, TimerMode::Repeating),
			max_enemy_count: 4,
		})
		.add_plugin(MinimapPlugin)
		// Startup Systems
		.add_startup_system(setup)
		.add_startup_system(setup_player)
		.add_system(player_shoot)
		// Enemies
		// .add_startup_system(spawn_boss)
		.add_system(update_level_using_gems)
		.add_system(spawn_random_enemy)
		.add_system(danger_hit_player)
		.add_system(tick_down_player_invincibility)
		.add_system(unspawn_dead_mobs)
		.add_system(run_wraith)
		.add_system(run_goo)
		.add_system(run_boss)
		.add_system(run_ranger)
		.add_system(animate_ranger_sprite)
		.add_system(projectile_hit_mobs)
		.add_system(mob_face_movement_sprite_sheet)
		.add_system(mob_face_movement_sprite)
		.add_system(despawn_far_enemies)
		// Player, Projectiles
		.add_system(update_velocity)
		.add_system(update_select)
		.add_system(animate_player_sprite)
		.add_system(update_cooldowns)
		.add_system(update_crystals_velocity)
		.add_system(update_mines_velocity)
		.add_system(despawn_old_projectiles)
		// Tiles
		.add_system(spawn_tiles)
		.add_system(despawn_tiles)
		.add_system(update_tiles)
		.add_system(simulator_step)
		.add_system(update_lightmap)
		.add_stage_after(CoreStage::Update, DESPAWN_STAGE, SystemStage::single_threaded())
		.add_system_to_stage(DESPAWN_STAGE, despawn)
		.add_system_to_stage(CoreStage::PostUpdate, update_camera)
		.add_system(player_collect_gem)
		.run();
}

fn setup(
	mut commands: Commands,
	mut atlases: ResMut<Atlases>,
	asset_server: Res<AssetServer>,
	mut simulator: ResMut<Simulator>,
	mut texture_atlases: ResMut<Assets<TextureAtlas>>,
	mut rapier_configuration: ResMut<RapierConfiguration>,
) {
	rapier_configuration.gravity = Vec2::ZERO;

	setup_camera(&mut commands);

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
	atlases.campfire_atlas = texture_atlases.add(TextureAtlas::from_grid(
		asset_server.load("campfire.png"),
		Vec2::new(16., 16.),
		4,
		1,
		None,
		None,
	));
	simulator.post_init();
}

pub fn check_if_dead(players: Query<&Player>, mut app_state: ResMut<State<AppState>>) {
	for player in players.iter() {
		if player.health <= 0 {
			app_state.set(AppState::Dead).unwrap_or(());
			return;
		}
	}
}

pub fn enter_dead(mut commands: Commands, mut app_state: ResMut<State<AppState>>) {
	println!("Entering dead");
	app_state.clear_schedule();
	app_state.set(AppState::Alive).unwrap_or(());
	return;
}

pub fn reset_vars(
	mut commands: Commands,
	asset_server: Res<AssetServer>,
	mut simulator: ResMut<Simulator>,
	mut set: ParamSet<(
		Query<(&mut Transform, &mut Player)>,
		Query<(Entity, &EnemyBoss)>,
		Query<(Entity, &Mob)>,
		Query<(Entity, &Projectile)>,
	)>,
) {
	println!("Restting vars");
	for (mut transform, mut player) in set.p0().iter_mut() {
		*transform = Transform::from_translation(Vec3::new(32. * MAP_RADIUS as f32, 32. * MAP_RADIUS as f32, 2.));
		*player = Player {
			level: 0,
			health: MAX_HEALTH,
			invincibility_seconds: 2.0,
			gem_count: 0,
			select: PlayerWeaponSelect::Firebolt,
		};
	}
	*simulator = Simulator::new(
		MAP_RADIUS * 2,
		(3, 6),
		(10, MAP_RADIUS - 6),
		(10, 13),
		15,
		(20, 30),
		2,
		10,
		20,
		5,
		20,
	);
	simulator.post_init();
	for (boss, _) in set.p1().iter() {
		commands.entity(boss).insert(Despawn);
	}
	//boss::spawn_boss(&mut commands, &asset_server, Vec2::new(0.0, 0.0));
	for (mob, _) in set.p2().iter() {
		commands.entity(mob).insert(Despawn);
	}
	for (projectile, _) in set.p3().iter() {
		commands.entity(projectile).insert(Despawn);
	}
}

pub fn simulator_step(
	mut commands: Commands,
	mut simulator: ResMut<Simulator>,
	player: Query<&Transform, With<Player>>,
	structures: Query<(Entity, &Transform), With<Structure>>,
	mut timer: ResMut<SimulatorTimer>,
	time: Res<Time>,
	keyboard_input: Res<Input<KeyCode>>,
	atlases: Res<Atlases>,
	asset_server: Res<AssetServer>,
) {
	let player_trans = player.single().translation.truncate();
	let player_pos = position_to_tile_position(&player_trans);
	timer.0.tick(time.delta());
	if keyboard_input.just_pressed(KeyCode::E) {
		// Spawn boss if close to boss spawner
		let boss_room_loc = simulator.boss_room_loc();
		if boss_room_loc.as_vec2().distance(player_pos.as_vec2()) < 5. {
			spawn_boss(&mut commands, &asset_server, _tile_position_to_position(&boss_room_loc))
		} else if simulator.grid.campfires.contains(&player_pos) {
			simulator.remove_campfire(player_pos);
			for (e, t) in structures.iter() {
				let structure_trans = t.translation.truncate();
				if position_to_tile_position(&structure_trans) == player_pos {
					commands.entity(e).insert(Despawn);
				}
			}
		} else {
			simulator.place_campfire(player_pos);
			spawn_campfire_sprite(&mut commands, &atlases, player_pos);
		}
	}
	if timer.0.just_finished() {
		simulator.step(player_pos);
	}
}

pub fn despawn(mut commands: Commands, despawns: Query<Entity, With<Despawn>>) {
	for entity in despawns.iter() {
		commands.entity(entity).despawn();
	}
}
