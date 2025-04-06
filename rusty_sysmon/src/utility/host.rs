use sysinfo::System;

pub struct HostUtil{}

impl HostUtil {
    // initial function to avoid duplicate code
    fn refreshed_system() -> System {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys
    }
}