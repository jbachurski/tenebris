use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Velocity(Vec2);

#[derive(Component)]
pub struct Acceleration {
    max_velocity: f32,
    rate: f32
}

pub fn setup_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        Velocity(Vec2::new(0.0, 0.0)),
        Acceleration {max_velocity: 10.0, rate: 2.0},
        SpriteBundle {
            texture: asset_server.load("test.png"),
            ..default()
        }
    ));
}

pub fn update_velocity(keyboard_input: Res<Input<KeyCode>>, mut query: Query<(&mut Velocity, &Acceleration)>) {
    let (mut velocity, acceleration) = query.single_mut();
    let velocity_vec = &mut velocity.0;

    let mut acceleration_vec = Vec2::ZERO;
    let mut passive_deceleration = Vec2::ZERO;

    if keyboard_input.pressed(KeyCode::A) {
        acceleration_vec += Vec2::NEG_X;
        velocity_vec.x = f32::min(0.0, velocity_vec.x);
    }
    else if keyboard_input.pressed(KeyCode::D) {
        acceleration_vec += Vec2::X;
        velocity_vec.x = f32::max(0.0, velocity_vec.x);
    }
    else {
        passive_deceleration.x = -velocity_vec.x;
    }

    if keyboard_input.pressed(KeyCode::W) {
        acceleration_vec += Vec2::Y;
        velocity_vec.y = f32::max(0.0, velocity_vec.y);
    }
    else if keyboard_input.pressed(KeyCode::S) {
        acceleration_vec += Vec2::NEG_Y;
        velocity_vec.y = f32::min(0.0, velocity_vec.y);
    }
    else {
        passive_deceleration.y = -velocity_vec.y;
    }

    passive_deceleration = passive_deceleration.clamp_length_max(acceleration.rate * 2.0);

    acceleration_vec = acceleration_vec.clamp_length_max(acceleration.rate);
    *velocity_vec = (*velocity_vec + acceleration_vec + passive_deceleration).clamp_length_max(acceleration.max_velocity);
}

pub fn move_player(mut query: Query<(&mut Transform, &Velocity)>) {
    let (mut transform, velocity) = query.single_mut();

    transform.translation += velocity.0.extend(0.0);
}
