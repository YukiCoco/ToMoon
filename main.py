import pathlib
import subprocess
import asyncio
import os
import logging

HOME_DIR = str(pathlib.Path(os.getcwd()).parent.parent.resolve())
PARENT_DIR = str(pathlib.Path(__file__).parent.resolve())
logging.basicConfig(filename="/tmp/tomoon.py.log",
                    format='[tomoon] %(asctime)s %(levelname)s %(message)s',
                    filemode='w+',
                    force=True)
logger=logging.getLogger()
logger.setLevel(logging.INFO) # can be changed to logging.DEBUG for debugging issues

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