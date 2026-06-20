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

def build():
    # TODO: Implement building logic
    log.info("Checking components for building...")
    check_component("build")
    log.info("Building Rust bootstrap...")
    os.system("cargo run")

def parse_args():
    # The arg parser of this tool.
    parser = argparse.ArgumentParser(description="ProkaOS builder")
    subparsers = parser.add_subparsers(dest="subcmd", required=False)
    sub_build = subparsers.add_parser("build", help="Build kernel ISO image")
    sub_cfg = subparsers.add_parser("menuconfig", help="Config parser")
    sub_clean = subparsers.add_parser("clean", help="Clean up the file which was built")
    return parser.parse_args()

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
