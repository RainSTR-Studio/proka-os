use std::env;
use log::info;
use colored::Colorize;

fn main() {
    // Init logger
    env_logger::init();

    // Decide the build profile
    let key = "PROFILE";
    unsafe {
        if let Some(arg) = env::args().nth(1) {
            if arg == "release" {
                env::set_var(key, "release");
                info!("Will use {} mode to build projects!", "release".cyan().bold())
            } else {
                info!("Will use {} mode to build projects!", "debug".cyan().bold())
            }
        } else {
            info!("Will use {} mode to build projects!", "debug".cyan().bold())
        }
    }
}
