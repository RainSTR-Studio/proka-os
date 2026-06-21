//! Rust bootstrap of the kernel.
use colored::Colorize;
use log::{error, info};
use std::env;
use std::process::Command;
use std::process::exit;
use proka_builder::{arrange_iso, pack_iso};
use std::path::Path;

/// The directories which needs to iterate
const DIRS: [&'static str; 2] = ["bootloader", "kernel"];

fn main() -> std::io::Result<()> {
    // Init logger
    env_logger::init();

    // Decide the build profile
    let key = "PROFILE";
    unsafe {
        if let Some(arg) = env::args().nth(1) {
            if arg == "release" {
                env::set_var(key, "release");
                info!(
                    "Will use {} mode to build projects!",
                    "release".cyan().bold()
                )
            } else {
                info!("Will use {} mode to build projects!", "debug".cyan().bold())
            }
        } else {
            info!("Will use {} mode to build projects!", "debug".cyan().bold())
        }
    }

    // Iterate each directories and build
    for dir in DIRS {
        info!("Entering directory \"{}\"", dir.green().bold());
        let path = Path::new(dir);
        if !path.exists() {
            error!("This path does not exist!");
            exit(1)
        }

        let result = Command::new("make")
            .arg("-C")
            .arg(dir)
            .status()
            .expect("Failed to execute command");

        if !result.success() {
            error!("This path does not exist!");
            exit(result.code().expect("Failed to get code"))
        }

        info!("Build complete");
    }

    info!("Start to arrange ISO structure...");
    arrange_iso()?;
    info!("ISO arrangement completed.");

    info!("Preparing to pack ISO...");
    let path = pack_iso()?;
    info!("Successfully packes an ISO file at {}.", path.display());

    Ok(())
}
