use bevy::prelude::*;

#[derive(Component)]
pub struct Bounded {
	pub size: Vec2, // Radius of the bounding box.
}

#[derive(Component)]
pub struct Mob {
	pub health: u32,
}

#[derive(Component)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct Acceleration {
	pub max_velocity: f32,
	pub rate: f32,
}

pub fn move_by_velocity(mut entities: Query<(&mut Transform, &Velocity)>) {
	for (mut transform, velocity) in entities.iter_mut() {
		transform.translation += velocity.0.extend(0.0);
	}
}
