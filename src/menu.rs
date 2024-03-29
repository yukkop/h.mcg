use bevy::prelude::*;

use crate::{Map, setup_map};
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
        app.add_systems(Startup, setup)
            .add_systems(Update, (button_system, start_button_system, menu_call));
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    start_menu_query: Query<Entity, With<StartMenu>>,
) {
    commands.spawn(Camera2dBundle::default());
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
) {
    // TODO nrv explain pls
    // let start_interaction = interaction_query.single();
    for start_interaction in &interaction_query {
        match start_interaction {
            Interaction::Pressed => {
                let start_menu_entity = start_menu_query.single();
                commands.entity(start_menu_entity).despawn();
                if map_query.is_empty() {
                    setup_map(&mut commands);                   
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
                    // size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::SpaceBetween,
                    ..default()
                },
                ..default()
            })
            .with_children(|parent| {
                // left vertical fill (border)
                parent
                    .spawn(ButtonBundle {
                        style: Style {
                            // size: Size::new(Val::Px(150.0), Val::Px(65.0)),
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
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
            })
            .insert(StartMenu);
    }
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor /* , &Children */),
        (Changed<Interaction>, With<Button>),
    >,
    // mut text_query: Query<&mut Text>,
) {
    for (interaction, mut color /* , children */) in &mut interaction_query {
        // let mut text = text_query.get_mut(children[0]).unwrap();
        match *interaction {
            Interaction::Pressed => {
                // text.sections[0].value = "Press".to_string();
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                // text.sections[0].value = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                // text.sections[0].value = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
