#!/usr/bin/env python3
from builder.chk import check_component
from builder.logger import init_log, log
import argparse
import os
import sys
import shutil

# Directories to build/clean
dirs = ["bootloader", "kernel"]

def cfg():
    log.info("Checking components for configuration...")
    if not check_component("menuconfig"):
        log.critical("One component wasn't found, please install it.")
        sys.exit(1)
    log.info("Loading configurator...")
    os.system("cargo anaxa menuconfig")

def build(is_debug: bool):
    log.info("Checking components for building...")
    if not check_component("build"):
        log.critical("One component wasn't found, please install it.")
        sys.exit(1)
    log.info("Building Rust bootstrap...")
    if is_debug:
        code = os.system("RUST_LOG=info cargo run") != 0
        if code != 0:
            log.critical("Process did not exit successfully")
            sys.exit(1)
    else:
        code = os.system("RUST_LOG=info cargo run release")
        if code != 0:
            log.critical("Process did not exit successfully")
            sys.exit(1)

def clean():
    for dir in dirs:
        log.info(f"Cleaning up \"{dir}\"...")
        os.system(f"make -C {dir} clean")
    log.info("Cleaning up bootstrap...")
    os.system("cargo clean")
    log.info("Cleaning up the ISO and its build dir...")
    shutil.rmtree(r"./iso")
    os.remove("proka.iso")

def parse_args():
    # The arg parser of this tool.
    parser = argparse.ArgumentParser(description="ProkaOS builder")
    subparsers = parser.add_subparsers(dest="subcmd", required=False)
    sub_build = subparsers.add_parser("build", help="Build kernel ISO image")
    subparsers.add_parser("menuconfig", help="Build configurator")
    sub_build.add_argument("-d", "--debug", action="store_true", help="Build as debug profile")
    subparsers.add_parser("clean", help="Clean up the file which was built")
    return parser.parse_args()

def pull_submod():
    os.system("git submodule sync")
    os.system("git submodule update --init --recursive --depth=1")

def main():
    # Init logger
    init_log()

    # Pull down submodules
    log.info("Pulling down submodules for further operation...")
    pull_submod()

    # Parse arguments then
    args = parse_args()
    if args.subcmd is None:
        build(False)
    elif args.subcmd == "build":
        if args.debug:
            build(True)
        else:
            build(False)
    elif args.subcmd == "menuconfig":
        cfg()
    elif args.subcmd == "clean":
        clean()

if __name__ == "__main__":
    main()
