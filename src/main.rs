use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod prelude;
mod cheats;

use log::{info, debug};

// Base pointer
pub const LOCAL_PLAYER: u32 = 0x0017E0A8;

// TODO! Convert all u32 to usize

fn main() {
    env_logger::builder()
        .filter_level(log::LevelFilter::Debug)
        .format_target(false)
        .format_timestamp(None)
        .init();

    let process: Process = Process::new("ac_client.exe");

    debug!("{:?}", &process);

    let local_player = sdk::player::Player::new(process.clone(), process.base_address as u32 + LOCAL_PLAYER);

    info!("Local player found: 0x{:x}", local_player.address);
    info!("health: {}", local_player.health());
    info!("armor: {}", local_player.armor());
    info!("assault rifle ammo: {}", local_player.assault_rifle_ammo());

    // Entity list does not work currently. I need to restructure it.

    //let mut entity_list = sdk::entity_list::EntityList::new(process.clone());
    //info!("Entity list found: 0x{:x}", entity_list.address);
}
