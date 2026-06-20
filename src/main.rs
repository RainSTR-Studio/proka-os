//! Rust bootstrap of the kernel.
use colored::Colorize;
use log::{error, info};
use std::env;
use std::path::Path;
use std::process::Command;
use std::process::exit;

/// The directories which needs to iterate
const DIRS: [&'static str; 2] = ["bootloader", "kernel"];

fn main() {
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

    // TODO: Intergrate them
}
