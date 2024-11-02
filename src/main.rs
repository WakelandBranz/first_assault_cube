use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;
mod types;

fn main() {
    let process: Process = Process::new("ac_client.exe");
    println!("Process:\n{:?}", &process);
}
