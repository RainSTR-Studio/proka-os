# Logger for this bootstrap
import logging
import sys

def init_log():
    log_format = "[%(levelname)s] %(message)s"
    logging.basicConfig(
        format=log_format,
        level=logging.INFO,
        datefmt="%H:%M:%S",
        stream=sys.stdout
    )

log = logging.getLogger(__name__)
