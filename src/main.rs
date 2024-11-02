use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod prelude;

fn main() {
    let process: Process = Process::new("ac_client.exe");

    println!("{:?}", &process);

    let player = sdk::player::Player::new(process);

    println!("{:?}", player);

    loop {
        println!("{}",player.health());
        std::thread::sleep(std::time::Duration::from_millis(100));
    }
}
