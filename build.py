import argparse
import logging
import sys
import os
import shutil

def init_log():
    log_format = "[%(levelname)s] %(message)s"
    logging.basicConfig(
        format=log_format,
        level=logging.INFO,
        datefmt="%H:%M:%S",
        stream=sys.stdout
    )

def cmd_check(cmd: str) -> bool:
    return shutil.which(cmd) is not None

# Global logger
log = logging.getLogger(__name__)

# TODO: Move this to config file
dirs = ["bootloader", "kernel"]

def build():
    # TODO: Implement building logic
    log.info("Checking components for building...")
    check_component("build")

def cfg():
    log.info("Checking components for configuration...")
    check_component("menuconfig")
    log.info("Loading configurator...")
    os.system("cargo anaxa menuconfig")

def parse_args():
    # The arg parser of this tool.
    parser = argparse.ArgumentParser(description="ProkaOS builder")
    subparsers = parser.add_subparsers(dest="subcmd", required=False)
    sub_build = subparsers.add_parser("build", help="Build kernel ISO image")
    sub_cfg = subparsers.add_parser("menuconfig", help="Config parser")
    sub_clean = subparsers.add_parser("clean", help="Clean up the file which was built")
    return parser.parse_args()

def check_component(typ: str) -> bool:
    # Check the component which is needed in building process
    # This depends on the type which passed in.
    # Will return false if check not succeed.
    
    # First of all, check the MOST essential component: Rust
    # Here, cargo and rustc are required.
    # 
    # Sometimes, rustup is not installed
    if not (cmd_check("cargo") or cmd_check("rustc")):
        log.error("\"cargo\" or \"rustc\" are missed.")
        return False

    # For building, these components are all required
    match typ:
        case "build":
            # We need to check these:
            #  - Rustup
            #  - NASM
            #  - GCC (idk is clang ok)
            if not cmd_check("rustup"):
                log.error("\"rustup\" not found")
                return False
            if not cmd_check("nasm"):
                log.error("\"nasm\" not found")
                return False
            if not cmd_check("gcc"):
                log.error("\"GCC\" not found")
                return False
        case "menuconfig":
            # Here, the cargo-anaxa is required.
            if not cmd_check("cargo-anaxa"):
                os.system("cargo install cargo-anaxa")

def main():
    # Init logger
    init_log()

    # Parse arguments then
    args = parse_args()
    if args.subcmd in (None, "build"):
        build()
    elif args.subcmd in "menuconfig":
        cfg()

if __name__ == "__main__":
    main()
