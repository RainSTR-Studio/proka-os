//! Rust bootstrap of the kernel.
use colored::Colorize;
use log::{error, info};
use std::env;
use std::process::Command;
use std::process::exit;
use proka_builder::{arrange_iso, pack_iso};
use std::path::Path;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// The directories which needs to iterate
const DIRS: [&'static str; 2] = ["bootloader", "kernel"];

fn main() -> anyhow::Result<()> {
    // Init logger
    env_logger::init();

    // Decide the build profile
    let key = "PROFILE";
    let mut is_debug: bool = false;
    unsafe {
        if let Some(arg) = env::args().nth(1) {
            if arg == "release" {
                env::set_var(key, "release");
                info!(
                    "Will use {} mode to build projects!",
                    "release".cyan().bold()
                );
            } else {
                info!("Will use {} mode to build projects!", "debug".cyan().bold());
                is_debug = true;
            }
        } else {
            info!("Will use {} mode to build projects!", "debug".cyan().bold());
            is_debug = true;
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
        env::set_current_dir(path)?;

        let result = Command::new("./build.py")
            .status()
            .expect("Failed to execute command");

        if !result.success() {
            error!("Command doea not executed correctly!");
            exit(result.code().expect("Failed to get code"))
        }
        env::set_current_dir(Path::new(".."))?;

        info!("Build complete");
    }

    info!("Start to arrange ISO structure...");
    arrange_iso()?;
    info!("ISO arrangement completed.");

    // Get version through file
    let file = File::open("version")?;
    let reader = BufReader::new(file);
    let version = reader.lines().nth(0).ok_or(anyhow::anyhow!("unknown"))??;

    info!("Preparing to pack ISO...");
    let path = pack_iso(is_debug, &version)?;
    info!("Successfully packes an ISO file at {}.", path.display());

    Ok(())
}
