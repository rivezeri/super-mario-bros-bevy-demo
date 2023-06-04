use bevy::prelude::*;
use bevy::window::Window;

pub const PLAYER_SPEED: f32 = 275.0;
pub const PLAYER_SIZE:f32 = 32.0;

pub const FALL_SPEED:f32 = 300.0;

pub const ENEMY_SPEED:f32 = 50.0;
pub const ENEMY_SIZE:f32 = 64.0;

mod components;
use components::*;

mod map;
use map::*;

mod player;
use player::*;

mod enemy;
use enemy::*;

fn main() {
    App::new()
    .add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Mario 1-1 Demo".into(),
            resizable: false,
            resolution: (1280., 480.).into(),
            ..default()
        }),
        ..default()
    }))

    .insert_resource(ClearColor(Color::rgb(92.0 / 255.0, 148.0 / 255.0, 252.0 / 255.0)))
    .add_startup_system(spawn_player)
    .add_startup_system(spawn_camera)
    .add_startup_system(spawn_enemies)
    .add_startup_system(playback_audio)
    .add_startup_system(spawn_map)
    .add_startup_system(spawn_hud)
    .add_system(player_movement)
    .add_system(confine_player_movement)
    .add_system(player_fall)
    .add_system(camera_track_player)
    .add_system(platform_detection)
    .add_system(player_jump)
    .add_system(enemy_movement)
    .add_system(enemy_plat_detection)
    .add_event::<CollisionEvent>()
    .run();    
}

/*

This was followed by tutorials and mountains of troubleshooting. I learned a lot from coordinates and
use of this function, therefore I will keep it.

This function is the single most horrifying thing I've ever worked on.

 */
fn platform_detection(
    platform_query: Query<(&Transform, &Sprite), With<Collider>>,
    mut player_query: Query<&mut Transform, (With<Player>, Without<Collider>)>
) {
    for mut player_transform in player_query.iter_mut() {
        let distance = player_transform
        .translation;

        // On death
        if distance[1] < 31.5 {
            player_transform.translation = Vec3::new(-2968.0, 94.0, 0.0);
        }

        // On win
        if distance[0] == 2964.0 && (distance[1] < 400.0 && distance[1] > 100.0) { 
            
            player_transform.translation = Vec3::new(-2968.0, 94.0, 0.0);

        }
    
        let player_translation = player_transform.translation;
    
        for (platform_transform, sprite) in platform_query.iter() {
            let platform_translation = platform_transform.translation;
    
            if let Some(custom_size) = sprite.custom_size {
                let player_half_width = PLAYER_SIZE / 2.0;
                let player_half_height = PLAYER_SIZE / 2.0;
                let platform_half_width = custom_size.x / 2.0;
                let platform_half_height = custom_size.y / 2.0;
    
                let player_left = player_translation.x - player_half_width;
                let player_right = player_translation.x + player_half_width;
                let player_bottom = player_translation.y - player_half_height;
                let player_top = player_translation.y + player_half_height;
    
                let platform_left = platform_translation.x - platform_half_width;
                let platform_right = platform_translation.x + platform_half_width;
                let platform_bottom = platform_translation.y - platform_half_height;
                let platform_top = platform_translation.y + platform_half_height;
    
                if player_bottom <= platform_top && player_top >= platform_bottom &&
                player_left <= platform_right && player_right >= platform_left {

                let horizontal_overlap = player_right.min(platform_right) - player_left.max(platform_left);
                let vertical_overlap = player_bottom.min(platform_top) - player_top.max(platform_bottom);

                if horizontal_overlap.abs() + 10.0 < vertical_overlap.abs() {

                    if player_translation.x < platform_translation.x {
                        let displacement = Vec3::new(-horizontal_overlap.abs() - 0.1, 0.0, 0.0);
                        player_transform.translation += displacement;
                    } else {
                        let displacement = Vec3::new(horizontal_overlap.abs() + 0.1, 0.0, 0.0);
                        player_transform.translation += displacement;
                    }
                }    
                else {
                    if player_translation.y < platform_translation.y {
                        player_transform.translation.y = platform_bottom - PLAYER_SIZE / 2.0;
                    } else {
                        player_transform.translation.y = platform_top + PLAYER_SIZE / 2.0;
                        
                    }
                }
            }
        }
    }
}
}

fn playback_audio(asset_server: Res<AssetServer>, audio: Res<Audio>) {
    audio.play_with_settings(
        asset_server.load("overworld_theme.ogg"),
        PlaybackSettings::LOOP.with_volume(0.05),
        // Demonstration only, I don't need to annihilate eardrums
    );
}

fn spawn_hud(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        TextBundle::from_section(
            "MARIO",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                left: Val::Px(40.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "000000",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                left: Val::Px(40.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "x00",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                left: Val::Px(300.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "WORLD",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(300.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "1-1",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                right: Val::Px(330.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "TIME",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        ) 
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(10.0),
                right: Val::Px(40.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

    commands.spawn((
        TextBundle::from_section(
            "400",
            TextStyle {
                font: asset_server.load("fonts/Pixel_NES.otf"),
                font_size: 35.0,
                color: Color::WHITE,
            },
        ) 
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(35.0),
                right: Val::Px(40.0),
                ..default()
            },
            ..default()
        }),
        ColorText,
    ));

}