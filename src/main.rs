use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod prelude;
mod cheats;

use log::{info, debug};
use crate::sdk::entity_list::EntityList;
use crate::sdk::player::PlayerManager;

// Base pointer
pub const LOCAL_PLAYER: u32 = 0x0017E0A8;

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(true)
        .format_timestamp_secs()
        .init();

    let process: Process = Process::new("ac_client.exe");
    let base = process.base_address;

    debug!("{:?}", &process);

    let local_player_ptr = process.read::<u32>(base as u32 + LOCAL_PLAYER).unwrap();
    let mut local_player = PlayerManager::new(process.clone(), local_player_ptr)
        .unwrap_or_else(|| panic!("Couldn't get local player!"));

    let mut entity_list: EntityList = EntityList::new(process.clone());
    entity_list.update_entities();

    info!("Local player found!");
    info!("name: {}", local_player.name());
    info!("health: {}", local_player.health());
    info!("armor: {}", local_player.armor());
    info!("position: {}", local_player.position());

    local_player.set_armor(200);

    info!("current ammo: {}", local_player.ammo());
    info!("weapon usage count: {}", local_player.weapon_usage_count());

    // TODO: Implement check for if player is alive!

    info!("Local player found!");

    let mut i = 1;
    loop {
        if local_player.update() == Some(()) {

            local_player.set_health(local_player.health() + 10);

            entity_list.update_entities();
            //info!("Updated entity list count: {} (iteration {})", entity_list.entity_count, i);
            i += 1;
            std::thread::sleep(std::time::Duration::from_nanos(1));
        };
    }
}
