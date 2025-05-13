use bevy::prelude::*;
use textbox::Textbox;

use super::{ClientGameState, oneshot::ClientOneshotSystems};

pub mod button;
pub mod textbox;

use button::ButtonEffect;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(textbox::TextboxPlugin);
        app.add_plugins(button::ButtonPlugin);

        app.add_systems(Startup, setup)
            .add_systems(OnExit(ClientGameState::MainMenu), hide_menu)
            .add_systems(OnEnter(ClientGameState::MainMenu), show_menu);
    }
}

#[derive(Component)]
struct MainMenu;

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
                flex_direction: FlexDirection::Column,
                ..default()
            },
        ))
        .with_children(|parent| {
            parent.spawn((
                Textbox { focused: true },
                BackgroundColor(textbox::FOCUSED_BOX),
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::new("127.0.0.1"),
                TextFont {
                    font: asset_server.load("fonts/sans.ttf"),
                    font_size: 33.0,
                    ..default()
                },
                TextColor(Color::srgb(0.9, 0.9, 0.9)),
            ));

            parent.spawn((
                Button,
                ButtonEffect {
                    system: systems.list["connect"],
                },
                Node {
                    width: Val::Px(150.0),
                    height: Val::Px(34.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                Text::new("Button"),
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
fn show_menu(mut query: Query<&mut Visibility, With<MainMenu>>) {
    for mut visibility in &mut query {
        *visibility = Visibility::Visible;
    }
}
