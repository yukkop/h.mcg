use bevy::prelude::*;

use crate::map::{Map, setup_map};


const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
struct StartMenu;

#[derive(Component, Debug, Default, Clone, Copy, Reflect)]
#[reflect(Component, Default)]
struct StartButton;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(setup)
            .add_system(button_system)
            .add_system(start_button_system)
            .add_system(menu_call);
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    start_menu_query: Query<Entity, With<StartMenu>>,
) {
    spawn_start_menu(&mut commands, &asset_server, &start_menu_query);
}

fn menu_call(
    keyboard_input: Res<Input<KeyCode>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    start_menu_query: Query<Entity, With<StartMenu>>,
) {
    if keyboard_input.pressed(KeyCode::Escape) {
        spawn_start_menu(&mut commands, &asset_server, &start_menu_query);
    }
}

fn start_button_system(
    mut commands: Commands,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>)>,
    start_menu_query: Query<Entity, With<StartMenu>>,
    map_query: Query<Entity, With<Map>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // TODO nrv explain pls
    // let start_interaction = interaction_query.single();
    for start_interaction in &interaction_query {
        match start_interaction {
            Interaction::Clicked => {
                let start_menu_entity = start_menu_query.single();
                commands.entity(start_menu_entity).despawn_recursive();
                if map_query.is_empty() {
                    // setup_map(&mut commands, &mut meshes, &mut materials);                   
                }
            }
            Interaction::Hovered => { /* Nothink */ }
            Interaction::None => { /* Nothink */ }
        }
    }
}

fn spawn_start_menu(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    start_menu_query: &Query<Entity, With<StartMenu>>,
) {
    if start_menu_query.is_empty() {
        commands
            .spawn(NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            })
            .insert(StartMenu)
            .with_children(|parent| {
                // left vertical fill (border)
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            // center button
                            margin: UiRect::all(Val::Auto),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    })
                    .insert(StartButton)
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Start",
                            TextStyle {
                                font: asset_server.load("fonts/FiraSans-Bold.ttf"),
                                font_size: 40.0,
                                color: Color::WHITE,
                            },
                        ));
                    });
            });
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
