# The trigger of menuconfig.
import os
from bootstrap.chk import check_component
from bootstrap.logger import log

def cfg():    
    log.info("Checking components for configuration...")
    check_component("menuconfig")
    log.info("Loading configurator...")
    os.system("cargo anaxa menuconfig")
