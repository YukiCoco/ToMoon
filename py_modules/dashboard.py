import os
from pathlib import Path

import decky_plugin
from config import logger

defalut_dashboard = os.path.join(
    decky_plugin.DECKY_PLUGIN_DIR, "bin", "core", "dashboard", "yacd-meta"
)
defalut_dashboard_list = [defalut_dashboard]


def get_dashboard_list():
    dashboard_list = []

    try:
        # 遍历 dashboard_dir 下深度 1 的路径, 如果存在 xxx/index.html 则认为是一个 dashboard
        dashboard_dir_path = Path(f"{decky_plugin.DECKY_PLUGIN_DIR}/bin/core/web")

        for path in dashboard_dir_path.iterdir():
            if path.is_dir() and (path / "index.html").exists():
                dashboard_list.append(str(path))

        custom_dashboard_dir_path = Path(
            f"{decky_plugin.DECKY_PLUGIN_SETTINGS_DIR}/dashboard"
        )
        # 如果 custom_dashboard_dir_path 不存在 创建
        if not custom_dashboard_dir_path.is_dir():
            custom_dashboard_dir_path.mkdir(parents=True)
        for path in custom_dashboard_dir_path.iterdir():
            if path.is_dir() and (path / "index.html").exists():
                dashboard_list.append(str(path))

        logger.info(f"get_dashboard_list: {dashboard_list}")

        return dashboard_list
    except Exception as e:
        logger.error(f"error during get_dashboard_list: {e}")
        return defalut_dashboard_list
