# The checker of the bootstrap.
import os
import shutil

def cmd_check(cmd: str) -> bool:
    return shutil.which(cmd) is not None

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

