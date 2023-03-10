use bevy::{prelude::*, render::render_resource::TextureFormat};
use image::{DynamicImage, ImageBuffer, Rgba};

use crate::{
	gems::Gem,
	player::{CrystalCooldownTimer, FireboltCooldownTimer, MineCooldownTimer, Player, PlayerWeaponSelect, MAX_HEALTH},
	tiles::position_to_tile_position,
	tilesim::Simulator,
	utils::{DEBUG_OMNISCIENCE, MAP_RADIUS, MINIMAP_SIZE},
};

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<TotalMinimap>()
			.add_startup_system(setup_total_minimap)
			.add_system(update_total_minimap)
			.add_system(update_player_health_indicators)
			.add_system(update_spell_cooldown_overlays)
			.add_system(update_spell_indicator)
			.add_system(update_gem_count);
	}
}

#[derive(Default, Resource)]
struct TotalMinimap {
	handle: Handle<Image>,
}

#[derive(Component)]
struct PlayerHealthIndicator {
	health_threshold: i32,
}

#[derive(Component)]
struct SpellIndicator;

#[derive(Component)]
struct GemCount;

#[derive(Component)]
struct SpellCooldownOverlay(PlayerWeaponSelect);

fn setup_total_minimap(
	asset_server: Res<AssetServer>,
	mut commands: Commands,
	mut assets: ResMut<Assets<Image>>,
	mut total_minimap: ResMut<TotalMinimap>,
) {
	total_minimap.handle = assets.add(Image::from_dynamic(
		DynamicImage::ImageRgba8(ImageBuffer::new(MAP_RADIUS * 2, MAP_RADIUS * 2)),
		true,
	));
	commands.spawn(ImageBundle {
		style: Style {
			size: Size::new(Val::Px(MINIMAP_SIZE), Val::Px(MINIMAP_SIZE)),
			position_type: PositionType::Absolute,
			position: UiRect {
				right: Val::Px(10.0),
				top: Val::Px(10.0),
				..default()
			},
			..default()
		},
		image: UiImage(total_minimap.handle.clone()),
		..default()
	});

	// Make hearts on the top-left.
	let heart_handle = asset_server.load("heart.png");

	for i in 0..10 {
		commands
			.spawn(ImageBundle {
				style: Style {
					size: Size::new(Val::Px(32. * 2.), Val::Px(32. * 2.)),
					position_type: PositionType::Absolute,
					position: UiRect {
						left: Val::Px(10.0 + 32. * (i as f32)),
						top: Val::Px(10.0),
						..default()
					},
					..default()
				},
				image: UiImage(heart_handle.clone()),
				..default()
			})
			.insert(Visibility { is_visible: true })
			.insert(PlayerHealthIndicator {
				health_threshold: (MAX_HEALTH / 10) * (1 + i),
			});
	}

	// Make gem on the bottom-left
	let gem_handle = asset_server.load("gem/Gems.png");

	commands.spawn(ImageBundle {
		style: Style {
			size: Size::new(Val::Px(32.), Val::Px(32.)),
			position_type: PositionType::Absolute,
			position: UiRect {
				left: Val::Px(10.0),
				bottom: Val::Px(10.0),
				..default()
			},
			..default()
		},
		image: UiImage(gem_handle.clone()),
		..default()
	});

	commands
		.spawn(
			TextBundle::from_section(
				"12",
				TextStyle {
					font: asset_server.load("fonts/DejaVuSans.ttf"),
					font_size: 16.0,
					color: Color::WHITE,
				},
			)
			.with_style(Style {
				size: Size::new(Val::Px(32.), Val::Px(32.)),
				margin: UiRect {
					top: Val::Auto,
					left: Val::Px(74.0),
					bottom: Val::Px(10.0),
					..default()
				},
				..default()
			}),
		)
		.insert(GemCount);

	// Add current spell indicator on the bottom-right
	commands
		.spawn(NodeBundle {
			style: Style {
				size: Size::new(Val::Px(68.), Val::Px(68.)),
				position_type: PositionType::Absolute,
				position: UiRect {
					right: Val::Px(8.0),
					bottom: Val::Px(8.0),
					..default()
				},
				..default()
			},
			background_color: BackgroundColor(Color::rgba_u8(255, 255, 255, 127)),
			z_index: ZIndex::Local(0),
			..default()
		})
		.insert(SpellIndicator);

	// Make spell icons on the bottom-right
	let fireball_handle = asset_server.load("painterly-spell-icons/fireball-red-1.png");
	let storm_handle = asset_server.load("painterly-spell-icons/ice-sky-3.png");
	let explosion_handle = asset_server.load("painterly-spell-icons/explosion-orange-2.png");

	for (i, &handle) in [&fireball_handle, &storm_handle, &explosion_handle].iter().enumerate() {
		commands.spawn(ImageBundle {
			style: Style {
				size: Size::new(Val::Px(64.), Val::Px(64.)),
				position_type: PositionType::Absolute,
				position: UiRect {
					right: Val::Px(10.0 + (64. + 10.) * ((2 - i) as f32)),
					bottom: Val::Px(10.0),
					..default()
				},
				..default()
			},
			image: UiImage(handle.clone()),
			z_index: ZIndex::Local(1),
			..default()
		});
	}

	// Make spell cooldown icons on the bottom right
	for (i, spell) in [
		PlayerWeaponSelect::Firebolt,
		PlayerWeaponSelect::Crystals,
		PlayerWeaponSelect::Mine,
	]
	.iter()
	.enumerate()
	{
		commands
			.spawn(NodeBundle {
				style: Style {
					size: Size::new(Val::Px(64.), Val::Px(0.)),
					position_type: PositionType::Absolute,
					position: UiRect {
						right: Val::Px(10.0 + (64. + 10.) * ((2 - i) as f32)),
						bottom: Val::Px(10.0),
						..default()
					},
					..default()
				},
				background_color: BackgroundColor(Color::rgba_u8(255, 255, 255, 192)),
				z_index: ZIndex::Local(2),
				..default()
			})
			.insert(SpellCooldownOverlay(spell.clone()));
	}
}

fn update_gem_count(asset_server: Res<AssetServer>, players: Query<&Player>, mut indicators: Query<(&mut Text, &GemCount)>) {
	let mut count = 0;

	for player in players.iter() {
		count = player.gem_count;
	}

	for (mut text, _) in indicators.iter_mut() {
		text.sections = vec![TextSection::new(
			count.to_string(),
			TextStyle {
				font: asset_server.load("fonts/DejaVuSans.ttf"),
				font_size: 16.0,
				color: Color::WHITE,
			},
		)];
	}
}

fn update_player_health_indicators(players: Query<&Player>, mut indicators: Query<(&mut Visibility, &PlayerHealthIndicator)>) {
	let mut player_health = 0;

	for player in players.iter() {
		player_health = player.health;
	}

	for (mut visibility, indicator) in indicators.iter_mut() {
		visibility.is_visible = if indicator.health_threshold <= player_health {
			true
		} else {
			false
		};
	}
}

fn update_spell_indicator(players: Query<&Player>, mut indicators: Query<&mut Style, With<SpellIndicator>>) {
	let mut spell_index = 0;
	for player in players.iter() {
		use crate::player::PlayerWeaponSelect::*;
		spell_index = match &player.select {
			Firebolt => 0,
			Crystals => 1,
			Mine => 2,
		};
	}

	for mut style in indicators.iter_mut() {
		style.position.right = Val::Px(8.0 + (2 - spell_index) as f32 * (64. + 10.));
	}
}

fn update_spell_cooldown_overlays(
	firebolt_timers: Query<&FireboltCooldownTimer>,
	storm_timers: Query<&CrystalCooldownTimer>,
	explosion_timers: Query<&MineCooldownTimer>,
	mut overlays: Query<(&mut Style, &SpellCooldownOverlay)>,
) {
	let mut firebolt_ratio = 0.0;
	let mut storm_ratio = 0.0;
	let mut explosion_ratio = 0.0;

	for FireboltCooldownTimer(timer) in firebolt_timers.iter() {
		if !timer.finished() {
			firebolt_ratio = 1.0 - (timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32());
		}
	}

	for CrystalCooldownTimer(timer) in storm_timers.iter() {
		if !timer.finished() {
			storm_ratio = 1.0 - (timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32());
		}
	}

	for MineCooldownTimer(timer) in explosion_timers.iter() {
		if !timer.finished() {
			explosion_ratio = 1.0 - (timer.elapsed().as_secs_f32() / timer.duration().as_secs_f32());
		}
	}

	for (mut style, overlay) in overlays.iter_mut() {
		use PlayerWeaponSelect::*;
		style.size.height = Val::Px(
			64. * match overlay.0 {
				Firebolt => firebolt_ratio,
				Crystals => storm_ratio,
				Mine => explosion_ratio,
			},
		);
	}
}

fn update_total_minimap(
	total_minimap: Res<TotalMinimap>,
	simulator: Res<Simulator>,
	mut assets: ResMut<Assets<Image>>,
	player_query: Query<&Transform, With<Player>>,
) {
	if let Some(image) = assets.get_mut(&total_minimap.handle) {
		let player_position = player_query.single().translation.truncate();
		let mut image_buffer = ImageBuffer::new(MAP_RADIUS * 2, MAP_RADIUS * 2);
		for (x, y, p) in image_buffer.enumerate_pixels_mut() {
			if DEBUG_OMNISCIENCE || simulator.grid.reality_bubble.contains(&UVec2::new(x, MAP_RADIUS * 2 - y - 1)) {
				*p = get_minimap_color(
					&simulator,
					position_to_tile_position(&player_position),
					x,
					MAP_RADIUS * 2 - y - 1,
				);
			} else {
				*p = Rgba([102, 102, 255, 25]);
			}
		}
		*image = Image::from_dynamic(DynamicImage::ImageRgba8(image_buffer), true);
	}
}

fn get_minimap_color(simulator: &Simulator, player_pos: UVec2, i: u32, j: u32) -> Rgba<u8> {
	let pos = UVec2::new(i, j);
	if pos.as_vec2().distance(player_pos.as_vec2()) < 2. {
		Rgba([255, 255, 255, 255])
	} else if simulator.grid.campfires.contains(&pos) {
		Rgba([102, 255, 102, 127])
	} else if simulator.grid.is_wall[i as usize][j as usize] {
		Rgba([102, 102, 255, 51])
	} else {
		Rgba([102, 102, 255, 127])
	}
}
