use super::components::*;
use super::resources::*;
use super::STARS_NUM;

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use rand::prelude::*;

pub fn spawn_star(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
) {
    let primary_window = window_query.get_single().unwrap();

    for _ in 0..STARS_NUM {
        let x_pos = random::<f32>() * primary_window.width();
        let y_pos = random::<f32>() * primary_window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                texture: asset_server.load("sprites/Default/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

// Unlike spawn_star does not creates STARS_NUM of stars, but instead periodicall spawns them.
pub fn spawn_stars_over_time(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    star_timer: Res<StarTimer>,
) {
    let primary_window = window_query.get_single().unwrap();

    if star_timer.timer.just_finished() {
        let x_pos = random::<f32>() * primary_window.width();
        let y_pos = random::<f32>() * primary_window.height();

        commands.spawn((
            SpriteBundle {
                transform: Transform::from_xyz(x_pos, y_pos, 0.0),
                texture: asset_server.load("sprites/Default/star.png"),
                ..default()
            },
            Star {},
        ));
    }
}

pub fn star_timer_tick(mut star_timer: ResMut<StarTimer>, time: Res<Time>) {
    star_timer.timer.tick(time.delta());
}
