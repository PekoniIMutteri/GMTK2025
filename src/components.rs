use avian2d::prelude::*;
use bevy::prelude::*;

#[derive(PhysicsLayer, Default)]
pub enum GameLayer {
    #[default]
    Default,
    Player,
    Enemy,
    Props,
    Node,
    Map,
}

#[derive(Component)]
#[require(
    MoveSpeed,
    DropNode,
    RigidBody::Dynamic,
    CollisionLayers::new(
        GameLayer::Player,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Props, GameLayer::Map],
    ),
    Collider,
    Sprite
)]
pub struct Player;

#[derive(Component)]
#[require(
    KillPlayer,
    MoveSpeed,
    AI,
    Collider,
    RigidBody::Dynamic,
    CollisionLayers::new(
        GameLayer::Enemy,
        [GameLayer::Default, GameLayer::Player, GameLayer::Enemy, GameLayer::Node, GameLayer::Map],
    ),
    Sprite,
)]
pub enum Enemy {
    Knight,
    Mage,
}

#[derive(Component)]
#[require(Sprite, Collider, RigidBody::Static,
    CollisionLayers::new(
        GameLayer::Node,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Map, GameLayer::Node],
    ),
)]
pub struct Node;

#[derive(Component)]
#[require(Sprite, Collider, RigidBody::Static,
    CollisionLayers::new(
        GameLayer::Node,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Map],
    ),
)]
pub struct Link;

#[derive(Component, Default)]
#[require(Transform, LinearVelocity)]
pub struct MoveSpeed(f32);

#[derive(Component, Default)]
pub struct DropNode;

#[derive(Component)]
#[require(MoveSpeed)]
pub struct Input {
    left: KeyCode,
    right: KeyCode,
    up: KeyCode,
    down: KeyCode,
    drop: KeyCode,
}

#[derive(Component, Default)]
#[require(Collider)]
pub struct KillPlayer;

#[derive(Component, Default)]
#[require(MoveSpeed)]
pub struct AI;
