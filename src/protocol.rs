use bevy::prelude::*;
use lightyear::prelude::ReplicationGroup;

pub mod component;
pub mod input;
pub mod message;

pub struct ProtocolPlugin;

impl Plugin for ProtocolPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        component::register_components(app);

        message::register_messages(app);

        input::register_input(app);
    }
}

pub const REPLICATION_GROUP: ReplicationGroup = ReplicationGroup::new_id(0);
