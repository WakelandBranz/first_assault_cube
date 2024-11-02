use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod prelude;

fn main() {
    env_logger::init();

    let process: Process = Process::new("ac_client.exe");

    println!("{:?}", &process);

    let local_player = sdk::player::Player::new(process);
    log::info!("health: {}", local_player.health());
    log::info!("armor: {}",local_player.armor());
    log::info!("grenade: {}",local_player.grenade_ammo());
    log::info!("assault rifle ammo: {}",local_player.assault_rifle_ammo());

    println!("{:?}", local_player);

    std::thread::sleep(std::time::Duration::from_millis(3000));

    local_player.set_fast_fire_ar(true).unwrap();
    local_player.set_fast_fire_sniper(true).unwrap();

    loop {
        println!("X: {}\nY: {}\nZ: {}",local_player.position_x(), local_player.position_y(), local_player.position_z());
        std::thread::sleep(std::time::Duration::from_millis(1));

        let curr_y = local_player.position_y();

        local_player.set_random_name().unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
        local_player.set_health(100).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
        local_player.set_assault_rifle_ammo(2).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
        local_player.set_fast_fire_ar(true).unwrap();
        local_player.set_fast_fire_sniper(true).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(1));
    }
}
