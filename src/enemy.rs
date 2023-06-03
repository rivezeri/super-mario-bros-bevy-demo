use bevy::prelude::*;

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