use bevy::prelude::{App, Camera2d, Color, Commands, Component, DefaultPlugins, KeyCode, Query, Res, Sprite, Startup, Time, Transform, Update, Vec2, With};
use bevy::asset::AssetServer;
use bevy::color::palettes::css::{BLUE, RED};
use bevy::sprite::{Anchor, SpriteImageMode};
use bevy::input::ButtonInput;
use rand::prelude::*;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Speed(f32);

#[derive(Component)]
struct Direction(Vec2);

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (player_movement, enemy_spawner, move_enemies))
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d::default());
   
    // Player as a blue circle
    commands.spawn((
        Sprite {
            color: Color::from(BLUE),
            custom_size: Some(Vec2::new(50.0, 50.0)),
            flip_x: false,
            flip_y: false,
            anchor: Anchor::Center,
            texture_atlas: None,
            rect: None,
            image: asset_server.load("sprites/game.png"),
            image_mode: SpriteImageMode::Auto,
        },
        Transform::from_xyz(0.0, -200.0, 0.0),
        Player,
        Speed(300.0),
    ));
}

fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    for mut transform in &mut player_query {
        let mut direction = 0.0;
       
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction += 1.0;
        }
       
        transform.translation.x += direction * 300.0 * time.delta().as_secs_f32();
    }
}

fn enemy_spawner(
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>
) {
    // Spawn enemy every 2 seconds
    if time.elapsed().as_secs_f32() % 2.0 < time.delta().as_secs_f32() {
        let mut rng = rand::rng();
        
        // Random spawn position at top of screen
        let spawn_x = rng.random_range(-400.0..400.0);
        
        // Random direction (normalized)
        let random_angle = rng.random_range(0.0..std::f32::consts::TAU);
        let direction = Vec2::new(random_angle.cos(), random_angle.sin()).normalize();
        
        commands.spawn((
            Sprite {
                color: Color::from(RED),
                custom_size: Some(Vec2::new(40.0, 40.0)),
                flip_x: false,
                flip_y: false,
                anchor: Anchor::Center,
                texture_atlas: None,
                rect: None,
                image: asset_server.load("sprites/game.png"),
                image_mode: SpriteImageMode::Auto,
            },
            Transform::from_xyz(spawn_x, 300.0, 0.0),
            Enemy,
            Speed(150.0),
            Direction(direction),
        ));
    }
}

fn move_enemies(
    mut enemy_query: Query<(&mut Transform, &Speed, &Direction), With<Enemy>>,
    time: Res<Time>,
) {
    for (mut transform, speed, direction) in &mut enemy_query {
        // Move enemy in its random direction
        transform.translation.x += direction.0.x * speed.0 * time.delta().as_secs_f32();
        transform.translation.y += direction.0.y * speed.0 * time.delta().as_secs_f32();
        
        // Remove enemies that go off screen (simple cleanup)
        if transform.translation.x.abs() > 800.0 || transform.translation.y.abs() > 600.0 {
        }
    }
}