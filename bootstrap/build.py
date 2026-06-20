# The builder of this bootstrap
from bootstrap.chk import check_component
from bootstrap.logger import log

def build():
    # TODO: Implement building logic
    log.info("Checking components for building...")
    check_component("build")
