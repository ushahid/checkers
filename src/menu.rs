//! This example illustrates the various features of Bevy UI.

use bevy::prelude::*;

use crate::{state::GameState, ai::AIStatus};



pub struct CheckersMenuPlugin;


impl Plugin for CheckersMenuPlugin {
    fn build(&self, app: &mut App){
        app
        .add_system_set(SystemSet::on_enter(GameState::Menu).with_system(setup))
        .add_system_set(SystemSet::on_update(GameState::Menu).with_system(button_system))
        .add_system_set(SystemSet::on_exit(GameState::Menu).with_system(cleanup))
        .run();
    }
}

#[derive(Resource)]
struct HoveredButtonColor(Color);


#[derive(Resource)]
struct ButtonColor(Color);


#[derive(Component)]
struct Menu;

fn cleanup(mut commands: Commands, query: Query<Entity, With<Menu>>, asset_server: Res<AssetServer>){
    for menu in query.iter() {
        commands.entity(menu).despawn_recursive();
    }
    asset_server.mark_unused_assets();
    asset_server.free_unused_assets();
}


fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &Children),
        (Changed<Interaction>, With<Button>)
    >,
    hovered_color: Res<HoveredButtonColor>,
    button_color: Res<ButtonColor>,
    text_query: Query<&Name>,
    mut game_state: ResMut<State<GameState>>,
    mut ai_status: ResMut<AIStatus>
) {
    for (interaction, mut color, children) in &mut interaction_query {
        let text = text_query.get(children[0]).unwrap();
        match *interaction {
            Interaction::Hovered => {
                *color = hovered_color.0.into();
            },
            Interaction::None => {
                *color = button_color.0.into();
            },
            Interaction::Clicked => {
                match text.as_str() {
                    "HUMAN" => {
                        ai_status.enabled = false;
                        game_state.set(GameState::BoardSetup).unwrap();
                    },
                    "CPU" => {
                        ai_status.enabled = true;
                        game_state.set(GameState::BoardSetup).unwrap();
                    },
                    _ => {}
                }
            }
        }
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    const BUTTON_BACKGROUND: Color = Color::rgb(0.1, 0.1, 0.1);
    const BUTTON_FONT_COLOR: Color = Color::rgb(0.9, 0.9, 0.9);
    const BUTTON_FONT_SIZE: f32 = 20.;
    const BUTTON_HEIGHT: f32 = 65.;
    const BUTTON_WIDTH_RELATIVE: f32 = 50.;
    let button_font: Handle<Font> = asset_server.load("fonts/MunichRegular.ttf");
    commands.insert_resource(HoveredButtonColor(Color::rgb(0.2, 0.2, 0.2)));
    commands.insert_resource(ButtonColor(BUTTON_BACKGROUND));


    // root node
    commands
        .spawn(NodeBundle {
            style: Style {
                size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            background_color: Color::BLACK.into(),
            ..default()
        })
        .insert(Menu)
        .with_children(|parent| {
                parent.spawn(NodeBundle {
                    style: Style {
                        size: Size::new(Val::Percent(30.0), Val::Percent(50.0)),
                        flex_direction: FlexDirection::Column,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    background_color: Color::BLACK.into(),
                    ..default()
                }).with_children(|parent|{


                    // logo image
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(100.0), Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(ImageBundle {
                            style: Style {
                                size: Size::new(Val::Px(500.0), Val::Auto),
                                ..default()
                            },
                            image: asset_server.load("images/logo.png").into(),
                            ..default()
                        });
                    });

                    // title
                    parent.spawn(NodeBundle {
                        style: Style {
                            size: Size::new(Val::Percent(80.), Val::Auto),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            margin: UiRect{top: Val::Px(50.), ..default()},
                            ..default()
                        },
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(
                            TextBundle::from_section(
                                "Checkers",
                                TextStyle {
                                    font: asset_server.load("fonts/Pixeboy.ttf"),
                                    font_size: 60.0,
                                    color: Color::rgb(0.8, 0.8, 0.8),
                                    ..default()
                                },
                            )
                        );
                    });
                    
                    // first button
                    parent.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(BUTTON_WIDTH_RELATIVE), Val::Px(BUTTON_HEIGHT)),
                            margin: UiRect{top: Val::Px(50.), ..default()},
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BUTTON_BACKGROUND.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Two Players",
                            TextStyle {
                                font: button_font.clone(),
                                font_size: BUTTON_FONT_SIZE,
                                color: BUTTON_FONT_COLOR.into(),
                            },
                        )).insert(Name::new("HUMAN"));
                    });

                    // second button
                    parent.spawn(ButtonBundle {
                        style: Style {
                            size: Size::new(Val::Percent(BUTTON_WIDTH_RELATIVE), Val::Px(BUTTON_HEIGHT)),
                            margin: UiRect{top: Val::Px(20.), ..default()},
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: BUTTON_BACKGROUND.into(),
                        ..default()
                    })
                    .with_children(|parent| {
                        parent.spawn(TextBundle::from_section(
                            "Against CPU",
                            TextStyle {
                                font: button_font.clone(),
                                font_size: BUTTON_FONT_SIZE,
                                color: BUTTON_FONT_COLOR.into(),
                            },
                        )).insert(Name::new("CPU"));
                    });
                });
                
        });
}