#!/usr/bin/python3
from bootstrap.chk import check_component
from bootstrap.logger import init_log, log
import argparse
import logging
import sys
import os
import shutil

def cfg():
    log.info("Checking components for configuration...")
    check_component("menuconfig")
    log.info("Loading configurator...")
    os.system("cargo anaxa menuconfig")

def build(is_debug: bool):
    # TODO: Implement building logic
    log.info("Checking components for building...")
    check_component("build")
    log.info("Building Rust bootstrap...")
    if is_debug:
        os.system("RUST_LOG=info cargo run")
    else:
        os.system("RUST_LOG=info cargo run release")

def parse_args():
    # The arg parser of this tool.
    parser = argparse.ArgumentParser(description="ProkaOS builder")
    subparsers = parser.add_subparsers(dest="subcmd", required=False)
    sub_build = subparsers.add_parser("build", help="Build kernel ISO image")
    sub_cfg = subparsers.add_parser("menuconfig", help="Config parser")
    sub_build.add_argument("-d", "--debug", action="store_true", help="Build as debug profile")
    sub_clean = subparsers.add_parser("clean", help="Clean up the file which was built")
    return parser.parse_args()

def pull_submod():
    os.system("git submodule update --init --recursive --depth=1")

def main():
    # Init logger
    init_log()

    # Pull down submodules
    log.info("Pulling down submodules for further operation...")

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

if __name__ == "__main__":
    main()
