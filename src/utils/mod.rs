use sysinfo::{ProcessExt, System, SystemExt};

pub fn get_pid_by_name(name: impl ToString) -> i32 {
    let s = System::new_all();

    for process in s.get_process_by_name("ac_client") {
        println!("{} {} ", process.pid(), process.name());
        if process.name() == name {
            process.pid()
        }
    }
}