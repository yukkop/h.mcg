use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use bevy_mod_picking::RaycastMesh;
use crate::net::{plugin::{node_system}, structs::{OldNode, Dot, DotBundle, LineBundle, NodeBundle}};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(node_system).add_startup_system(setup_map_system);
    }
}

pub fn setup_map_system(
    mut commands: Commands,
     mut meshes:  ResMut<Assets<Mesh>>,
     mut materials: ResMut<Assets<ColorMaterial>>,
) {
                    setup_map(&mut commands, &mut meshes, &mut materials);                   
}

//map chanking
pub fn setup_map(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials:&mut ResMut<Assets<ColorMaterial>>,
) {
    // Таблица интендентности
    //   a b c d
    // a 1      
    // b 0 1    
    // c 1 0 1  
    // d 0 1 0 1
    
    let intendent: Vec::<Vec::<i32>> = vec![
        //           1  2  3  4
        /* 1 */ vec![1, 1, 1, 0],
        /* 2 */ vec![1, 1, 0, 1],
        /* 3 */ vec![1, 0, 1, 0],
        /* 4 */ vec![0, 1, 0, 1],
    ];

    // вершины графа
    // [Vec2; 4]
    let nodes: Vec::<Vec2> = vec![
        /* 1 */ Vec2::new(100., 100.), 
        /* 2 */ Vec2::new(-100., 200.),
        /* 3 */ Vec2::new(300., 200.),
        /* 4 */ Vec2::new(200., -150.)
    ];                    

    commands.spawn(SpriteBundle {
        sprite: Sprite::default(),
        transform: Transform::from_translation(Vec3::new(0., 0., 1.)),
        ..default()
    }).add_children(|parent| {
            for (index_i, node) in nodes.iter().enumerate() {

                // spawn Node
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::from_translation(Vec3::new(
                       node.x,  // pos x
                       node.y,  // pos y
                       2., // layer
                    )).with_scale(Vec3::splat(12.)),
                    material: materials.add(ColorMaterial::from(ColorMaterial::from(Color::hex("ffffff").unwrap()))),
                    ..default()
                })
                .insert(RaycastMesh::<crate::net::structs::Node>::default());

                for index_j in 0..index_i {
                    if intendent[index_i][index_j] > 0 {
                        let start_position = nodes[index_i];
                        let end_position = nodes[index_j];

                        let vec_ = end_position - start_position;
                        let middle_point = (vec_ / 2.) + start_position;
                        let length = vec_.length();

                        let angle = Vec2::angle_between(Vec2::new(1., 0.), vec_);

                    parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::hex("e60c0c").unwrap(),
                    custom_size: Some(Vec2::new(length, 7.)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    middle_point.x,
                    middle_point.y,
                     1 as f32,
                ))
                .with_rotation(Quat::from_axis_angle(Vec3::new(0., 0., 1.), angle)),
                ..default()
            },);
        }
                }
            }
        });
}

//map chanking
pub fn setup_map_old(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials:&mut ResMut<Assets<ColorMaterial>>,
) {
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

                // spawn Node
                parent.spawn(MaterialMesh2dBundle {
                    mesh: meshes.add(Mesh::from(shape::Quad::default())).into(),
                    transform: Transform::from_translation(Vec3::new(
                       node.dot.x,
                       node.dot.y,
                       2 as f32,
                    )).with_scale(Vec3::splat(12.)),
                    material: materials.add(ColorMaterial::from(ColorMaterial::from(Color::hex("ffffff").unwrap()))),
                    ..default()
                })
                .insert(RaycastMesh::<crate::net::structs::Node>::default());

                for connection in node.connections {
                    // spawn Line
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