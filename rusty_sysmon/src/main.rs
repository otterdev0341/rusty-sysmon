slint::include_modules!();

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AddIt::new()?;
//     ui.run()
// }

use std::{thread, time::Duration};

use netstat2::{get_sockets_info, AddressFamilyFlags, ProtocolFlags, ProtocolSocketInfo, TcpState};
use rusty_sysmon::utility::{convert_helper::ConvertHelper, cpu::CpuUtill, disk::DiskUtill, host::HostUtil, network::NetworkUtill, process::ProcessUtill, ram::RamUtil};
use sysinfo::System;


fn main() -> Result<(), slint::PlatformError> {
    let ui = NetworkPage::new()?;
    ui.run()?;

    Ok(())
    
   
    

}

        





