use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Default)]
pub struct CollisionEvent;

#[derive(Component)]
pub struct Enemy {
    pub direction: Vec2,
}

#[derive(Resource)]
pub struct CollisionSound(Handle<AudioSource>);

#[derive(Component)]
pub struct Collider;

#[derive(Resource)]
pub struct Score (u32);

#[derive(Component)]
pub struct Jump(pub f32);

#[derive(Component)]
pub struct Ball;

#[derive(Component, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component)]
pub struct ColorText;

#[derive(Component)]
pub struct AnimationIndices {
    pub first: usize,
    pub last: usize,
}

#[derive(Component, Deref, DerefMut)]
pub struct AnimationTimer(pub Timer);

#[derive(Component)]
pub struct MyComponent {
    pub has_started : bool,
}