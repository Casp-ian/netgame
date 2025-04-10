use bevy::{ecs::system::SystemId, prelude::*};

use super::{ClientGameState, oneshot::ClientOneshotSystems};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, button_system)
            .add_systems(OnEnter(ClientGameState::Game), hide_menu);
    }
}

#[derive(Component)]
struct MainMenu;

#[derive(Component)]
#[require(Button)]
pub struct ButtonEffect {
    system: SystemId,
}

const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

fn button_system(
    mut commands: Commands,
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor, &mut Text, &ButtonEffect),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color, mut text, effect) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                **text = "Press".to_string();
                *color = PRESSED_BUTTON.into();
                commands.run_system(effect.system);
            }
            Interaction::Hovered => {
                **text = "Hover".to_string();
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                **text = "Button".to_string();
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    systems: Res<ClientOneshotSystems>,
) {
    // ui camera
    commands
        .spawn((
            MainMenu,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Button,
                ButtonEffect {
                    system: systems.list["connect"],
                },
                Node {
                    width: Val::Px(150.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                BackgroundColor(NORMAL_BUTTON),
                Text::new("Button"),
                // TextLayout {
                //     justify: JustifyText::Center,
                //     linebreak: LineBreak::NoWrap,
                // },
                TextFont {
                    font: asset_server.load("fonts/sans.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));
        });
}

fn hide_menu(mut query: Query<&mut Visibility, With<MainMenu>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Hidden;
    }
}
