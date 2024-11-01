use crate::process::Process;

mod sdk;
mod process;
mod feature;
mod utils;

fn main() {
    let pid: i32 = utils::get_pid_by_name("ac_client");
    println!("Pid: {}", pid);

    let process: Process = Process::new(pid);
    println!("Process:\n{:?}", &process);
}
