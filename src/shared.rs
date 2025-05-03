use bevy::prelude::*;
use lightyear::prelude::{SharedConfig, TickConfig};
use std::time::Duration;

pub mod casting;
pub mod enemy;
pub mod map;
pub mod player;
pub mod projectile;

pub const REPLICATION_INTERVAL: Duration = Duration::from_millis(33);
pub const TICK_DURATION: Duration = Duration::from_millis(33);

pub struct SharedPlugins;

impl Plugin for SharedPlugins {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            casting::CastingPlugin,
            enemy::EnemyPlugin,
            map::MapPlugin,
            player::PlayerPlugin,
        ));
    }
}

pub fn shared_config() -> SharedConfig {
    SharedConfig {
        server_replication_send_interval: REPLICATION_INTERVAL,
        client_replication_send_interval: REPLICATION_INTERVAL,
        tick: TickConfig {
            tick_duration: TICK_DURATION,
        },
    }
}
