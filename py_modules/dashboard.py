import os
import decky_plugin
from config import logger

defalut_dashboard_list = [os.path.join(decky_plugin.DECKY_PLUGIN_DIR, "bin", "core", "web", "yacd")]

def get_dashboard_list():
    dashboard_list = []

    try:
        dashboard_dir = os.path.join(decky_plugin.DECKY_PLUGIN_DIR, "bin", "core", "web")
        # 遍历 dashboard_dir 下的路径, 如果存在 xxx/index.html 则认为是一个 dashboard
        for root, dirs, files in os.walk(dashboard_dir):
            for dir in dirs:
                if os.path.exists(os.path.join(root, dir, "index.html")):
                    dashboard_list.append(dir)

        custom_dashboard_dir = os.path.join(decky_plugin.DECKY_PLUGIN_SETTINGS_DIR, "web")
        # 遍历 custom_dashboard_dir 下的路径, 如果存在 xxx/index.html 则认为是一个 dashboard
        for root, dirs, files in os.walk(custom_dashboard_dir):
            for dir in dirs:
                if os.path.exists(os.path.join(root, dir, "index.html")):
                    dashboard_list.append(dir)

        return dashboard_list
    except Exception as e:
        logger.error(f"error during get_dashboard_list: {e}")
        return defalut_dashboard_list