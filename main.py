import pathlib
import subprocess
import asyncio
import os

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())


class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded

    async def _main(self):
        # startup
        self.backend_proc = subprocess.Popen([PARENT_DIR + "/bin/backend"])
        while True:
            await asyncio.sleep(1)
