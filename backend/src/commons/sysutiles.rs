use sysinfo::{System, SystemExt};

#[allow(dead_code)]
pub fn process_exists(pid: &i32) -> bool {
    let mut sys = System::new_all();
    sys.refresh_all();
    for (syspid, _) in sys.processes() {
        if syspid.to_string().eq(&pid.to_string()) {
            return true;
        }
    }
    return false;
}
