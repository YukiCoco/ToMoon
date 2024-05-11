import os
import decky_plugin
from pathlib import Path
from config import logger

defalut_dashboard = os.path.join(
    decky_plugin.DECKY_PLUGIN_DIR, "bin", "core", "dashboard", "yacd"
)
defalut_dashboard_list = [defalut_dashboard]

dashboard_link_path = Path(f"{decky_plugin.DECKY_PLUGIN_DIR}/bin/core/web")


def get_dashboard_list():
    dashboard_list = []

    try:
        # 遍历 dashboard_dir 下深度 1 的路径, 如果存在 xxx/index.html 则认为是一个 dashboard
        dashboard_dir_path = Path(f"{decky_plugin.DECKY_PLUGIN_DIR}/bin/core/dashboard")

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

        return dashboard_list
    except Exception as e:
        logger.error(f"error during get_dashboard_list: {e}")
        return defalut_dashboard_list


def get_current_dashboard():
    # 判断 dashboard_link_path 是否存在，并且是否是一个软链接，如果是软连接，返回软连接的目标
    if dashboard_link_path.exists() and dashboard_link_path.is_symlink():
        return os.path.realpath(dashboard_link_path)
    else:
        return ""


def set_dashboard(dashboard_path):
    logger.info(f"set_dashboard: {dashboard_path}")
    # 判断 dashboard_path 是否存在，并且是否是一个目录
    if not Path(dashboard_path).is_dir():
        return False

    # 判断 dashboard_link_path 是否存在，如果存在删除
    if dashboard_link_path.exists():
        dashboard_link_path.unlink()

    # 创建软链接
    os.symlink(dashboard_path, dashboard_link_path)

    return True


def set_default_dashboard(skip_if_exists=True):
    if not skip_if_exists or not get_current_dashboard():
        logger.info("set_default_dashboard")
        set_dashboard(defalut_dashboard)
        return True
    return False
