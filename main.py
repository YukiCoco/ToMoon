import logging
import pathlib
import subprocess
import asyncio
import os

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())

LOG_LOCATION = "/tmp/clashdeck.py.log"


logging.basicConfig(
    filename=LOG_LOCATION,
    format='%(asctime)s %(levelname)s %(message)s',
    filemode='w',
    force=True)

logger = logging.getLogger()
logger.setLevel(logging.DEBUG)
logging.info(f"ClashDeck is running")


class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded

    async def _main(self):
        # startup
        self.backend_proc = subprocess.Popen([PARENT_DIR + "/bin/backend"])
        while True:
            await asyncio.sleep(1)
