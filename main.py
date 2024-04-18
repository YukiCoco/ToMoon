import subprocess
import asyncio
import os
from config import logger,PARENT_DIR
import update

class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        logger.info("Start Tomoon.")
        os.system('chmod -R 777 ' + PARENT_DIR)
        # 切换到工作目录
        os.chdir(PARENT_DIR)
        self.backend_proc = subprocess.Popen([PARENT_DIR + "/bin/tomoon"])
        while True:
            await asyncio.sleep(1)
    
    # Function called first during the unload process, utilize this to handle your plugin being removed
    async def _unload(self):
        logger.info("Stop Tomoon.")
        self.backend_proc.kill()
        pass

    async def update_latest(self):
        logger.info("Updating latest")
        return update.update_latest()

    async def get_version(self):
        return update.get_version()

    async def get_latest_version(self):
        return update.get_latest_version()