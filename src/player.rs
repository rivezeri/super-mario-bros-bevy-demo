use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::window::Window;



pub fn player_jump(
    mut commands: Commands,
    mut player: Query<(Entity, &mut Transform, &mut super::Jump), With<super::Player>>,
    time: Res<Time>,
) {
    let Ok((player, mut transform,mut jump)) = player.get_single_mut() else {return;};
    let jump_power = (time.delta_seconds() * super::FALL_SPEED * 1.5).min(jump.0);
    jump.0 -= jump_power;
    transform.translation.y += jump_power;
    if jump.0 == 0. {
        commands.entity(player).remove::<super::Jump>();
    }
}

pub fn player_fall(
    mut player: Query<&mut Transform, (With<super::Player>, Without<super::Jump>)>,
    time: Res<Time>,
) {
        let Ok(mut player) = player.get_single_mut()
        else {return;};
        if player.translation.y > 0.0 {
            player.translation.y -= time.delta_seconds() * super::FALL_SPEED;
            if player.translation.y < 0.0 { player.translation.y = 0.0 }
        }

}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands : Commands,
    mut player: Query<(Entity, &mut Transform), With<super::Player>>,
    time: Res<Time>,
) {
    let (entity, mut player) = player.single_mut();
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        player.translation.x -= super::PLAYER_SPEED * time.delta_seconds();

        if (keyboard_input.pressed(KeyCode::LShift)
        || keyboard_input.pressed(KeyCode::RShift)) && (!keyboard_input.pressed(KeyCode::Space) && !keyboard_input.pressed(KeyCode::Up)) {
            player.translation.x -= super::PLAYER_SPEED * time.delta_seconds() * 1.2;
    }

    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode:: D) {
        player.translation.x += super::PLAYER_SPEED * time.delta_seconds();

        if (keyboard_input.pressed(KeyCode::LShift)
        || keyboard_input.pressed(KeyCode::RShift)) && (!keyboard_input.pressed(KeyCode::Space) && !keyboard_input.pressed(KeyCode::Up)) {
            player.translation.x += super::PLAYER_SPEED * time.delta_seconds() * 1.2;
    }

    }
    if keyboard_input.pressed(KeyCode::Space) || keyboard_input.pressed(KeyCode::Up) {
        commands.entity(entity).insert(super::Jump(100.));
    }
}

pub fn confine_player_movement(
    mut player_query: Query<&mut Transform, With<super::Player>>,
    window_query: Query<&Window, With<PrimaryWindow>>,
) {
    if let Ok(mut player_transform) = player_query.get_single_mut() {
        let window = window_query.get_single().unwrap();

        let half_player_size: f32 = super::PLAYER_SIZE / 2.0;
        let x_min = half_player_size - 3000.0;
        let x_max = window.width() - half_player_size + 1700.00;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        let mut translation = player_transform.translation;

        if translation.y < y_min {
            translation.y = y_min;
        }
        else if translation.y > y_max {
            translation.y = y_max;
        }

        if translation.x < x_min {
            translation.x = x_min;
        }
        else if translation.x > x_max {
            translation.x = x_max;
        }

        player_transform.translation = translation;

    }
}

pub fn spawn_player (
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
)
    {
        let window = window_query.get_single().unwrap();

        commands.spawn((
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-3000.00, window.height() - window.height(), 0.0),
                    scale: Vec3::new(2.0, 2.0, 1.0), 
                    ..Default::default()
                },
                texture: asset_server.load("mario.png"),
                ..Default::default()
            },
            super::Player,
        ));

    }

pub fn spawn_camera(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
)
{
    let window = window_query.get_single().unwrap();

    commands.spawn(
        Camera2dBundle {
            transform : Transform::from_xyz(-3000.00, window.height() - window.height(), 0.0),
            ..default()
        }
    );
}

pub fn camera_track_player(
    mut camera_transform: Query<&mut Transform, With<Camera>>,
    player_transform: Query<&Transform, (With<super::Player>, Without<Camera>)>,
) {
    let mut camera_trans = camera_transform.single_mut();
    let playertrans = player_transform.single().translation.truncate();
    let camtrans = camera_trans.translation.truncate() + 10.82;

    if playertrans[1] > 31.0 {

        let limit_height = Vec2::new(playertrans[0], 32.0);
        camera_trans.translation = camtrans.lerp(limit_height, 0.05).extend(999.0);
    }
}