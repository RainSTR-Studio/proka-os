#!/usr/bin/python3
from bootstrap.build import build
from bootstrap.menucfg import cfg
from bootstrap.logger import init_log, log
import argparse
import logging
import sys
import os
import shutil

# TODO: Move this to config file
dirs = ["bootloader", "kernel"]

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
