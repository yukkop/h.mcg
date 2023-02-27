use std::{rc::Rc, sync::Arc};
use bevy::{input::mouse::MouseMotion, ui::node_bundles};
use bevy_mod_picking::*;


pub(crate) use bevy::prelude::*;

#[derive(Component)]
struct Node {
    dot: Dot,
    connections: Vec<Dot>,
}

#[derive(Component)]
struct Net(Vec<Node>);

struct NetBundle {
    sprite: SpriteBundle,
    net: Net,
}

impl NetBundle {
    fn new(a_net: Net, layer: u32) -> Self {
        Self {
            net: a_net,
            sprite: SpriteBundle {
                sprite: Sprite::default(),
                transform: Transform::from_translation(Vec3::new(0., 0., layer as f32)),
                ..default()
            },
        }
    }
}

#[derive(Component, Deref)]
struct Size(Vec2);

#[derive(Component, Deref)]
struct Frame(Vec2);

#[derive(Component)]
pub struct Map;

#[derive(Component, Deref, Clone, Copy)]
struct Dot(Vec2);

#[derive(Component)]
struct Line;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(node_system);
    }
}

#[derive(Component)]
struct Hoverable2D {
    hovered: bool,
}

//map chanking
pub fn setup_map(commands: &mut Commands) {
    // let pos1 = Vec2::new(100., 200.);
    // let pos2 = Vec2::new(-500., -100.);
    //
    // let map = MapBundle::new(Vec2::ZERO, Size(Vec2::new(1000., 1000.)), 0);
    //
    // commands.spawn(map).with_children(|child_builder| {
    //     child_builder.spawn(DotBundle::new(pos1, 1));
    //     child_builder.spawn(DotBundle::new(pos2, 1));
    //     child_builder.spawn(LineBundle::new(pos1, pos2, 5., 2));
    // });
    
    let mut nodes = Vec::new(); //{Dot(Vec2::new(100., 100.))};
    nodes.push(Node { dot: Dot(Vec2::new(100., 100.)), connections: Vec::new()});
    nodes.push(Node { dot: Dot(Vec2::new(-100., 200.)), connections: Vec::new()});
    nodes.push(Node { dot: Dot(Vec2::new(300., 200.)), connections: Vec::new()});
    nodes.push(Node { dot: Dot(Vec2::new(200., -150.)), connections: Vec::new()});
    nodes.push(Node { dot: Dot(Vec2::new(150., -100.)), connections: Vec::new()});
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

#[derive(Bundle)]
struct MapBundle {
    sprite: SpriteBundle,
    map: Map,
}

fn node_system(
    mut interaction_query: Query<
        (&Interaction, &Transform),
        (Changed<Interaction>, With<Node>),
    >,
) {
    println!("interaction for nodes is empty? {}", interaction_query.is_empty());
    for (interaction, transform) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                println!("Node Clicked");
            }
            Interaction::Hovered => {
                println!("Node Hovered");
            }
            Interaction::None => {
            }
        }
    }
}

impl MapBundle {
    fn new(position: Vec2, size: Size, layer: u32) -> Self {
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

#[derive(Bundle)]
struct LineBundle {
    sprite: SpriteBundle,
    line: Line,
}

impl LineBundle {
    fn new(start_pos: Vec2, end_pos: Vec2, width: f32, layer: u32) -> Self {
        let vec_ = end_pos - start_pos;
        let middle_point = (vec_ / 2.) + start_pos;
        let length = vec_.length();

        let angle = Vec2::angle_between(Vec2::new(1., 0.), vec_);

        Self {
            line: Line,
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("e60c0c").unwrap(),
                    custom_size: Some(Vec2::new(length, width)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    middle_point.x,
                    middle_point.y,
                    layer as f32,
                ))
                .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)),
                ..default()
            },
        }
    }
}

#[derive(Bundle)]
struct DotBundle {
    sprite: SpriteBundle,
    dot: Dot,
}

impl DotBundle {
    fn new(position: Vec2, layer: u32) -> Self {
        Self {
            dot: Dot(position),
            sprite: SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("ffffff").unwrap(),
                    custom_size: Some(Vec2::new(8., 8.)),
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
