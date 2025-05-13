use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use lightyear::{
    prelude::client::{ClientConnection, NetClient, Predicted},
    shared::replication::components::Controlled,
};

use crate::protocol::{component::PlayerId, input::NetworkedInput};

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostUpdate, handle_new_character);
    }
}

fn handle_new_character(
    connection: Res<ClientConnection>,
    mut commands: Commands,
    mut character_query: Query<(Entity, Has<Controlled>), (Added<Predicted>, With<PlayerId>)>,
) {
    for (entity, is_controlled) in &mut character_query {
        if is_controlled {
            info!("Adding InputMap to controlled and predicted entity {entity:?}");
            commands
                .entity(entity)
                .insert((InputMap::<NetworkedInput>::new([
                    (NetworkedInput::Jump, KeyCode::Space),
                    (NetworkedInput::Fire, KeyCode::KeyQ),
                ])
                .with_dual_axis(NetworkedInput::Move, VirtualDPad::wasd())
                .with_dual_axis(NetworkedInput::Look, MouseMove::default()),));
        } else {
            info!("Remote character replicated to us: {entity:?}");
        }
        let _client_id = connection.client.id();
        // info!(?entity, ?client_id, "Adding physics to character");
        // commands.entity(entity).insert(PlayerBundle::default());
    }
}
