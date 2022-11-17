use anyhow::Result;
use dotenv::dotenv;
use std::{env, fs};

// Documentation here: http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf

fn main() -> Result<()> {
    dotenv().ok();

    let file = fs::read(env::var("rom")?)?;
    let name_char_range = 0x134..0x142;

    // Prints out the name of the game
    for i in name_char_range {
        print!(
            "{}",
            char::from_u32(file[i] as u32).expect("There is a problem"),
        );
    }

    Ok(())
}
