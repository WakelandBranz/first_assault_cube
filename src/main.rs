use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod prelude;
mod cheats;

use log::{info, debug};
use crate::sdk::player::PlayerManager;

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
    let base = process.base_address;

    debug!("{:?}", &process);

    let local_player_ptr = process.read::<u32>(base as u32 + LOCAL_PLAYER).unwrap();
    let mut local_player = PlayerManager::new(process.clone(), local_player_ptr)
        .unwrap_or_else(|| panic!("Couldn't get local player!"));

    info!("Local player found!");
    info!("name: {}", local_player.name());
    info!("health: {}", local_player.health());
    info!("armor: {}", local_player.armor());
    info!("position: {}", local_player.position());

    local_player.set_armor(200);

    info!("current ammo: {}", local_player.ammo());
    info!("weapon usage count: {}", local_player.weapon_usage_count());

    let health_address = local_player_ptr + 0xEC;
    process.write::<u32>(health_address, 121);

    // TODO: Implement check for if player is alive!

    loop {
        if let res = local_player.update() == Some(()) {
            info!("Local player found!");
            info!("name: {}", local_player.name());
            info!("health: {}", local_player.health());
            info!("armor: {}", local_player.armor());
            info!("position: {}", local_player.position());
            info!("ammo: {}", local_player.ammo());
            local_player.set_ammo(69).unwrap();
            local_player.set_health(local_player.health() + 10);
            std::thread::sleep(std::time::Duration::from_millis(500));
        };
    }
}
