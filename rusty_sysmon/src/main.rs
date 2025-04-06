// slint::include_modules!();

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AddIt::new()?;
//     ui.run()
// }

use std::{thread, time::Duration};

use rusty_sysmon::utility::{cpu::CpuUtill, host::HostUtil, ram::RamUtil};
use sysinfo::System;


fn main() {
    
    // for i in 1..=5 {
    //     println!("Ram used : {:.2}", RamUtil::get_used_ram_gb());
    //     thread::sleep(Duration::from_secs(i));
    // }

    // println!("done after 5 secodes");

    // let host_name = HostUtil::get_os_name();
    // println!("{}", host_name);

    // let kernel_version = HostUtil::get_kernel_version();
    // println!("{}", kernel_version);

    // let os_version = HostUtil::get_os_version();
    // println!("{}", os_version);

    // let os_host_name = HostUtil::get_host_name();
    // println!("{}", os_host_name);

    let cpu_core = CpuUtill::get_cpu_core();
    println!("cpu core of this machine {:?} core.", cpu_core);
    let trait_check = CpuUtill::get_thread_count();
    println!("thread count of this machine {:?} thread.", trait_check);
   



}
