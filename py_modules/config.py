import logging


def setup_logger():
    logging.basicConfig(
        level=logging.INFO,
        filename="/tmp/tomoon.py.log",
        format="[%(asctime)s | %(filename)s:%(lineno)s:%(funcName)s] %(levelname)s: %(message)s",
        filemode="w+",
        force=True,
    )
    return logging.getLogger()


logger = setup_logger()

# can be changed to logging.DEBUG for debugging issues
logger.setLevel(logging.INFO)

API_URL = "https://api.github.com/repos/YukiCoco/ToMoon/releases/latest"

CONFIG_KEY = "tomoon"
