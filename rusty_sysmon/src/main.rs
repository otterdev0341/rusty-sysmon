// slint::include_modules!();

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AddIt::new()?;
//     ui.run()
// }

use std::{thread, time::Duration};

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use rusty_sysmon::utility::{convert_helper::ConvertHelper, cpu::CpuUtill, disk::DiskUtill, host::HostUtil, network::NetworkUtill, ram::RamUtil};
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

    // let cpu_core = CpuUtill::get_cpu_core();
    // println!("cpu core of this machine {:?} core.", cpu_core);
    // let trait_check = CpuUtill::get_thread_count();
    // println!("thread count of this machine {:?} thread.", trait_check);
   
   // Network
//    let network = NetworkUtill::get_host_ipv6();
//    println!("{}", network);

    // let data = NetworkUtill::get_network_traffic_per_second();
    // for thing in data.data {
    //     println!("device {}, upload: {} kb, download {} kb", thing.interface_name, thing.upload_per_sec, thing.download_per_sec)
    // }
    
    // let port_openned = NetworkUtill::get_allow_port_list();
    // for x in port_openned{
    //     println!("allow on port: {}",x);
    // }

   let data = DiskUtill::get_disk_data(2);
   println!("{:?}", data);
    let disk_capacity_gb = data.disk_capacity_byte / 1_073_741_824;
    print!("capcacity of disk is : {:.2} GB", disk_capacity_gb);

}

        





