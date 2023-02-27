use bevy::{prelude::*};
use crate::net::{plugin::node_system, structs::{OldNode, Dot, DotBundle, LineBundle}};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(node_system);
    }
}

//map chanking
pub fn setup_map(commands: &mut Commands) {
   
    let mut nodes = Vec::new(); //{Dot(Vec2::new(100., 100.))};
    nodes.push(OldNode { dot: Dot(Vec2::new(100., 100.)), connections: Vec::new()});
    nodes.push(OldNode { dot: Dot(Vec2::new(-100., 200.)), connections: Vec::new()});
    nodes.push(OldNode { dot: Dot(Vec2::new(300., 200.)), connections: Vec::new()});
    nodes.push(OldNode { dot: Dot(Vec2::new(200., -150.)), connections: Vec::new()});
    nodes.push(OldNode { dot: Dot(Vec2::new(150., -100.)), connections: Vec::new()});
    let dot = nodes[1].dot; 
    nodes[0].connections.push(dot);
    let dot = nodes[2].dot; 
    nodes[0].connections.push(dot);
    let dot = nodes[4].dot; 
    nodes[0].connections.push(dot);

    let dot = nodes[3].dot; 
    nodes[4].connections.push(dot);

    let dot = nodes[2].dot; 
    nodes[3].connections.push(dot);

    let dot = nodes[1].dot; 
    nodes[2].connections.push(dot);


    commands.spawn(SpriteBundle {
        sprite: Sprite::default(),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    }).add_children(|parent| {
            for node in nodes {
                parent.spawn(DotBundle::new(*node.dot, 2));
                for connection in node.connections {
                    parent.spawn(LineBundle::new(*node.dot, *connection, 7., 1));
                }
            }
        });
}

#[derive(Component)]
pub struct Map;

#[derive(Bundle)]
struct MapBundle {
    sprite: SpriteBundle,
    map: Map,
}

impl MapBundle {
    fn new(position: Vec2, size: crate::net::structs::Size, layer: u32) -> Self {
        Self {
            map: Map,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(1.0, 1.0, 1.0),
                    custom_size: Some(*size),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    position.x,
                    position.y,
                    layer as f32,

                )),
                ..default()
            },
        }
    }
}