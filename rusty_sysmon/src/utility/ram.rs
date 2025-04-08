use sysinfo::System;

use super::convert_helper::ConvertHelper;

pub struct RamUtil{}

impl RamUtil {

    // initial function to avoid duplicate code
    fn refreshed_system() -> System {
        let mut sys = System::new_all();
        sys.refresh_all();
        sys
    }

    pub fn get_total_ram_gb() -> f64 {
        
        let sys = Self::refreshed_system();
        // convert
        let ram = ConvertHelper::byte_to_gb(sys.total_memory());
        ram.round()

    }

    pub fn get_used_ram_gb() -> f64 {
        // initial
        let sys = Self::refreshed_system();
        // convert
        let used_ram = ConvertHelper::byte_to_gb(sys.used_memory());
        used_ram
        
    }

    pub fn get_swap_size() -> f64 {
        // initial
        let sys = Self::refreshed_system();

        let swap_size = ConvertHelper::byte_to_gb(sys.total_swap());
        swap_size
    }

    pub fn get_swap_used() -> f64 {
        // initial
        let sys = Self::refreshed_system();
        let swap_used = ConvertHelper::byte_to_gb(sys.used_swap());
        swap_used
    }

    pub fn get_swap_free() -> f64 {
        // initial
        let sys = Self::refreshed_system();
        let swap_free = ConvertHelper::byte_to_gb(sys.free_swap());
        swap_free
    }
}