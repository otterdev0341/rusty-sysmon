#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

slint::include_modules!();



fn main() -> Result<(), Box<dyn Error>> {
    // not error bec "rust-analyzer.procMacro.enable": true, in vs code
    let ui = AppEntry::new()?;
    ui.run()?;

    Ok(())
    
   
    

}

        





