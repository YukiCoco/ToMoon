import logging
import subprocess
import asyncio
import os
from config import logger, setup_logger
import update
import decky_plugin

class Plugin:
    backend_proc = None
    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        logger = setup_logger()
        logger.info("Start Tomoon.")
        os.system('chmod -R a+x ' + decky_plugin.DECKY_PLUGIN_DIR)
        # 切换到工作目录
        os.chdir(decky_plugin.DECKY_PLUGIN_DIR)
        self.backend_proc = subprocess.Popen([decky_plugin.DECKY_PLUGIN_DIR + "/bin/tomoon"])
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
        version = update.get_version()
        logger.info(f"Current version: {version}")
        return version

    async def get_latest_version(self):
        version = update.get_latest_version()
        logger.info(f"Latest version: {version}")
        return version