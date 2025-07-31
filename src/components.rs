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
    Input,
    Collider,
    CollisionLayers::new(
        GameLayer::Player,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Props, GameLayer::Map],
    ),
    RigidBody::Dynamic,
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
#[require(
    Collider,
    CollisionLayers::new(
        GameLayer::Node,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Map, GameLayer::Node],
    ),
    RigidBody::Static,
    Sprite,
)]
pub struct Node;

#[derive(Component)]
#[require(
    Collider,
    CollisionLayers::new(
        GameLayer::Node,
        [GameLayer::Default, GameLayer::Enemy, GameLayer::Map],
    ),
    RigidBody::Static,
    Sprite,
)]
pub struct Link;

#[derive(Component)]
#[require(
    Collider,
    CollisionLayers::new(
        GameLayer::Map,
        [
            GameLayer::Default,
            GameLayer::Player,
            GameLayer::Enemy,
            GameLayer::Props,
            GameLayer::Node,
            GameLayer::Map
        ],
    ),
    RigidBody::Static,
    Sprite,
)]
pub struct Map;

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

impl Default for Input {
    fn default() -> Self {
        Input {
            left: KeyCode::KeyA,
            right: KeyCode::KeyD,
            up: KeyCode::KeyW,
            down: KeyCode::KeyS,
            drop: KeyCode::Space,
        }
    }
}

#[derive(Component, Default)]
#[require(Collider)]
pub struct KillPlayer;

#[derive(Component, Default)]
#[require(MoveSpeed)]
pub struct AI;

#[derive(Component, Default)]
pub struct LastNode;

#[derive(Component, Default)]
pub struct FirstNode;
