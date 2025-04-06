use sysinfo::System;

pub struct HostUtil{}

impl HostUtil {
    // initial function to avoid duplicate code
    fn refreshed_system() -> System {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys
    }

    pub fn get_os_name() -> String {
        if let Some(data) = System::name() {
            return data;
        }
        String::from("Unknown Host")
    }

    pub fn get_kernel_version() -> String {
        if let Some(data) = System::kernel_version() {
            return data;
        }
        String::from("Can't get kernel information")
    }

    pub fn get_os_version() -> String {
        if let Some(data) = System::os_version() {
            return data
        }
        String::from("can't get os version")
    }

    pub fn get_host_name() -> String {
        if let Some(data) = System::host_name() {
            return data
        }
        String::from("can't get os host name")
    }
}