use bevy::prelude::*;

use crate::components::Enemy;

pub fn enemy_fall(
    mut enemy: Query<&mut Transform, (With<super::Enemy>, Without<super::Jump>)>,
    time: Res<Time>,
) {
        let Ok(mut enemy) = enemy.get_single_mut()
        else {return;};
        if enemy.translation.y > 0.0 {
            enemy.translation.y -= time.delta_seconds() * super::FALL_SPEED;
            if enemy.translation.y < 0.0 { enemy.translation.y = 0.0 }
        }

}

pub fn enemy_movement(mut enemy_query: Query<(&mut Transform, &super::Enemy)>, time: Res<Time>) {
    for (mut transform, enemy) in enemy_query.iter_mut() {
        let direction = Vec3::new(enemy.direction.x, 0., 0.0);
        transform.translation += direction * super::ENEMY_SPEED * time.delta_seconds();
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    player_query: Query<&mut Transform, (With<super::Player>, Without<super::Collider>)>,
    asset_server: Res<AssetServer>
) {
    commands.spawn(
        // I could keep adding on, but this is merely a demonsrtation
        (
            SpriteBundle {
                transform: Transform {
                    translation: Vec3::new(-2280.00, 76.0, 0.0),
                    scale: Vec3::new(2.0, 2.0, 1.0), 
                    ..Default::default()
                },
                texture: asset_server.load("goomba.png"),
                ..default()
            },
            super::Enemy {
                direction: Vec2::new(1., 1.).normalize(),
            },
        )
    );

}

pub fn enemy_plat_detection(
    platform_query: Query<(&Transform, &Sprite), With<super::Collider>>,
    mut enemy_query: Query<&mut Transform, (With<super::Enemy>, Without<super::Collider>)>,
    mut enemy: Query<&mut super::Enemy>
) {
    for mut enemy_id in enemy.iter_mut() {
        for enemy_transform in enemy_query.iter_mut() {
                let distance = enemy_transform
                .translation;

                // On death
                if distance[1] < 31.5 {
                    // despawn
                }
            
                let enemy_translation = enemy_transform.translation;
            
                for (platform_transform, sprite) in platform_query.iter() {
                    let platform_translation = platform_transform.translation;
            
                    if let Some(custom_size) = sprite.custom_size {
                        let enemy_half_width = super::ENEMY_SIZE / 2.0;
                        let enemy_half_height = super::ENEMY_SIZE / 2.0;
                        let platform_half_width = custom_size.x / 2.0;
                        let platform_half_height = custom_size.y / 2.0;
            
                        let enemy_left = enemy_translation.x - enemy_half_width;
                        let enemy_right = enemy_translation.x + enemy_half_width;
                        let enemy_bottom = enemy_translation.y - enemy_half_height;
                        let enemy_top = enemy_translation.y + enemy_half_height;
            
                        let platform_left = platform_translation.x - platform_half_width;
                        let platform_right = platform_translation.x + platform_half_width;
                        let platform_bottom = platform_translation.y - platform_half_height;
                        let platform_top = platform_translation.y + platform_half_height;
            
                        if enemy_bottom <= platform_top && enemy_top >= platform_bottom &&
                        enemy_left <= platform_right && enemy_right >= platform_left {

                        let horizontal_overlap = enemy_right.min(platform_right) - enemy_left.max(platform_left);
                        let vertical_overlap = enemy_bottom.min(platform_top) - enemy_top.max(platform_bottom);

                        if horizontal_overlap.abs() < vertical_overlap.abs() {

                            if enemy_translation.x < platform_translation.x {
                                enemy_id.direction *= -1.;
                            } else {
                                enemy_id.direction *= -1.;
                            }
                        }
                    }
                }
            }
        }
    }
}
