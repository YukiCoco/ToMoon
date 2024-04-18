import pathlib
import logging


PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())
logging.basicConfig(
    filename="/tmp/tomoon.py.log",
    format="[tomoon] %(asctime)s %(levelname)s %(message)s",
    filemode="w+",
    force=True,
)
logger = logging.getLogger()

# can be changed to logging.DEBUG for debugging issues
logger.setLevel(logging.INFO)

API_URL = "https://api.github.com/repos/YukiCoco/ToMoon/releases/latest"
