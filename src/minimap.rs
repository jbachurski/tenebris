use bevy::{prelude::*, render::render_resource::TextureFormat};
use image::{DynamicImage, ImageBuffer, Rgba};

use crate::{
	tilesim::Simulator,
	utils::{MAP_RADIUS, MINIMAP_SIZE},
};

pub struct MinimapPlugin;

impl Plugin for MinimapPlugin {
	fn build(&self, app: &mut App) {
		app.init_resource::<TotalMinimap>()
			.add_startup_system(setup_total_minimap)
			.add_system(update_total_minimap);
	}
}

#[derive(Default, Resource)]
struct TotalMinimap {
	handle: Handle<Image>,
}

fn setup_total_minimap(mut commands: Commands, mut assets: ResMut<Assets<Image>>, mut total_minimap: ResMut<TotalMinimap>) {
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
}

fn update_total_minimap(total_minimap: Res<TotalMinimap>, simulator: Res<Simulator>, mut assets: ResMut<Assets<Image>>) {
	if let Some(image) = assets.get_mut(&total_minimap.handle) {
		let mut image_buffer = ImageBuffer::new(MAP_RADIUS * 2, MAP_RADIUS * 2);
		for (x, y, p) in image_buffer.enumerate_pixels_mut() {
			if simulator.grid.reality_bubble.contains(&UVec2::new(x, MAP_RADIUS * 2 - y - 1)) {
				*p = get_minimap_color_but_better(&simulator, x, MAP_RADIUS * 2 - y - 1);
			} else {
				*p = Rgba([102, 102, 255, 25]);
			}
		}
		*image = Image::from_dynamic(DynamicImage::ImageRgba8(image_buffer), true);
	}
}

fn get_minimap_color_but_better(simulator: &Simulator, i: u32, j: u32) -> Rgba<u8> {
	if simulator.grid.is_wall[i as usize][j as usize] {
		Rgba([102, 102, 255, 51])
	} else {
		Rgba([102, 102, 255, 127])
	}
}
