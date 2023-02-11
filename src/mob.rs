use bevy::prelude::{Component, Vec2};

#[derive(Component)]
pub struct Bounded {
	pub size: Vec2, // Radius of the bounding box.
}

#[derive(Component)]
pub struct Mob {
	pub health: u32,
}