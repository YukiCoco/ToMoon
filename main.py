import asyncio
import os
import subprocess

import dashboard
import decky
import update
import utils
from config import logger, setup_logger
from settings import SettingsManager


class Plugin:
    backend_proc = None

    # Asyncio-compatible long-running code, executed in a task when the plugin is loaded
    async def _main(self):
        logger = setup_logger()

        self.settings = SettingsManager(
            name="config", settings_directory=decky.DECKY_PLUGIN_SETTINGS_DIR
        )

        utils.write_font_config()

        dashboard_list = dashboard.get_dashboard_list()
        logger.info(f"dashboard_list: {dashboard_list}")

        logger.info("Start Tomoon.")
        os.system("chmod -R a+x " + decky.DECKY_PLUGIN_DIR)
        # 切换到工作目录
        os.chdir(decky.DECKY_PLUGIN_DIR)
        self.backend_proc = subprocess.Popen([decky.DECKY_PLUGIN_DIR + "/bin/tomoon"])
        while True:
            await asyncio.sleep(1)

    # Function called first during the unload process, utilize this to handle your plugin being removed
    async def _unload(self):
        logger.info("Stop Tomoon.")
        self.backend_proc.kill()
        utils.remove_font_config()
        pass

    async def get_settings(self):
        return self.settings.getSetting(CONFIG_KEY)

    async def set_settings(self, settings):
        self.settings.setSetting(CONFIG_KEY, settings)
        logger.info(f"save Settings: {settings}")
        return True

    async def get_config_value(self, key):
        return self.settings.getSetting(key)

    async def set_config_value(self, key, value):
        self.settings.setSetting(key, value)
        logger.info(f"save config: {key} : {value}")
        return True

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

    async def get_dashboard_list(self):
        return dashboard.get_dashboard_list()
