use std::process::Command;

pub fn is_root() -> bool {
    unsafe { libc::getuid() == 0 }
}

pub fn stop_process(name: &str) {
    let _ = Command::new("killall").arg("-q").arg(name).status();
}
