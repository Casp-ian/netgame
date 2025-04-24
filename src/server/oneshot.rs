use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};

#[derive(Resource)]
pub struct ServerOneshotSystems {
    pub list: HashMap<String, SystemId>,
}

impl FromWorld for ServerOneshotSystems {
    fn from_world(world: &mut World) -> Self {
        let mut systems = ServerOneshotSystems {
            list: HashMap::new(),
        };

        systems
            .list
            .insert("stop".into(), world.register_system(super::network::stop));

        systems
    }
}
