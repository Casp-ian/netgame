use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct ClientOneshotSystems {
    pub list: HashMap<String, SystemId>,
}

impl FromWorld for ClientOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = ClientOneshotSystems {
            list: HashMap::new(),
        };

        systems.list.insert(
            "connect".into(),
            world.register_system(super::network::connect),
        );

        systems
    }
}
