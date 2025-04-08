use sysinfo::System;

pub struct HostUtil{}

impl HostUtil {

    pub fn get_os_name() -> Option<String> {
       match System::name() {
        Some(data) => Some(data),
        None => None
       }
        
    }

    pub fn get_kernel_version() -> Option<String> {
        match System::kernel_version() {
            Some(data) => Some(data),
            None => None
           }
    }

    pub fn get_os_version() -> Option<String> {
        match System::os_version() {
            Some(data) => Some(data),
            None => None
           }
    }

    pub fn get_host_name() -> Option<String> {
        match System::host_name() {
            Some(data) => Some(data),
            None => None
           }
    }
}