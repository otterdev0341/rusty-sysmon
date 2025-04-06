// slint::include_modules!();

// fn main() -> Result<(), slint::PlatformError> {
//     let ui = AddIt::new()?;
//     ui.run()
// }

use std::{thread, time::Duration};

use rusty_sysmon::utility::ram::RamUtil;


fn main() {
    
    for i in 1..=5 {
        println!("Ram used : {:.2}", RamUtil::get_used_ram_gb());
        thread::sleep(Duration::from_secs(i));
    }

    println!("done after 5 secodes");

}
