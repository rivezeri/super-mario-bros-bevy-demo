use bevy::prelude::*;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        // Spawn bg map
        SpriteBundle {
            transform: Transform::from_xyz(-2270.0,29.5,0.0),
            sprite: Sprite { custom_size: Some(Vec2::new(2208., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count last
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2672.0,189.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 256.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 7
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2627.0,268.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 6
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2595.0,236.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 5
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2563.0,204.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 4
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2531.0,172.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 3
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2499.0,140.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 2
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2467.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Count 1
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2435.0,76.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // Last platform
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2061.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(128., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Last pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2386.0,93.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1870.0,93.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 2..3th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1696.0,76.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 1..3th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1664.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 9..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1632.0,140.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 8..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1600.0,126.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 128.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 7..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1491.0,126.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 128.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 6..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1437.0,140.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 5..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1405.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 4..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1373.0,76.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 3..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1215.0,76.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 2..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1183.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 2..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1151.0,140.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 1..1th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1119.0,126.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 128.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 9..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(1023.0,126.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 128.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 8..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(991.0,104.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 7..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(959.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 6..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(927.0,76.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 5..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(128.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 4..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(785.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 3..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(785.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(128., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 2..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(540.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(96., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // 2..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(540.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(96., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // 1..th Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(414.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Tenth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(224.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Nineth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(127.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));


    // Eigth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(31.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Seventh Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-147.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(64., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Sixth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-350.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));
    
    // Sixth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-400.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(128., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Fifth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-688.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(256., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Fourth Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-865.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(96., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Third Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2655.0,300.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Second Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2655.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(160., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // First Block
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2850.0,173.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(32., 32.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Fourth pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-1520.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(60., 160.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // Third pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-1870.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(60., 160.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));
    

    // Second pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2127.0,108.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(60., 96.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    // First pipe
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-2448.0,93.0,0.0),
            visibility: Visibility::Hidden,
            sprite: Sprite { custom_size: Some(Vec2::new(60., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(2480.0,29.5,0.0),
            sprite: Sprite { custom_size: Some(Vec2::new(1792., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(498.0,29.5,0.0),
            sprite: Sprite { custom_size: Some(Vec2::new(2048., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(-865.0,29.5,0.0),
            sprite: Sprite { custom_size: Some(Vec2::new(480., 64.)),
            ..Default::default()
        },
        ..Default::default()
        },
        super::Collider,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("11.png"),
            transform: Transform::from_translation(Vec3::NEG_Y * 2.),
            sprite: Sprite { custom_size: Some(Vec2::new(6752., 960.)),
            ..Default::default()
        },
        ..Default::default()
        },
    ));

}