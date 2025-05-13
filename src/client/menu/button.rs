use bevy::{ecs::system::SystemId, prelude::*};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, button_system);
    }
}

#[derive(Component)]
#[require(Button)]
#[require(BackgroundColor = BackgroundColor(NORMAL_BUTTON))]
pub struct ButtonEffect {
    pub system: SystemId,
}

pub const NORMAL_BUTTON: Color = Color::srgb(0.15, 0.15, 0.15);
pub const HOVERED_BUTTON: Color = Color::srgb(0.25, 0.25, 0.25);
pub const PRESSED_BUTTON: Color = Color::srgb(0.35, 0.75, 0.35);

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
