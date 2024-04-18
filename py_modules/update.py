import json
import os
import shutil
import ssl
import stat
import subprocess

import urllib.request

from config import logger, API_URL
import decky_plugin


def recursive_chmod(path, perms):
    for dirpath, dirnames, filenames in os.walk(path):
        current_perms = os.stat(dirpath).st_mode
        os.chmod(dirpath, current_perms | perms)
        for filename in filenames:
            os.chmod(os.path.join(dirpath, filename), current_perms | perms)


def update_latest():
    downloaded_filepath = download_latest_build()

    if os.path.exists(downloaded_filepath):
        plugin_dir = decky_plugin.DECKY_PLUGIN_DIR

        try:
            logger.info(f"removing old plugin from {plugin_dir}")
            # add write perms to directory
            recursive_chmod(plugin_dir, stat.S_IWUSR)

            # remove old plugin
            shutil.rmtree(plugin_dir)
        except Exception as e:
            logger.error(f"ota error during removal of old plugin: {e}")

        try:
            logger.info(f"extracting ota file to {plugin_dir}")
            # extract files to decky plugins dir
            shutil.unpack_archive(
                downloaded_filepath,
                f"{decky_plugin.DECKY_USER_HOME}/homebrew/plugins",
                format="gztar",
            )

            # cleanup downloaded files
            os.remove(downloaded_filepath)
        except Exception as e:
            decky_plugin.logger.error(f"error during ota file extraction {e}")

        logger.info("restarting plugin_loader.service")
        cmd = "systemctl restart plugin_loader.service"
        result = subprocess.run(
            cmd,
            shell=True,
            check=True,
            text=True,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
        )
        logger.info(result.stdout)
        return result


def download_latest_build():
    gcontext = ssl.SSLContext()

    # response = urllib.request.urlopen(API_URL, context=gcontext)
    # json_data = json.load(response)

    # download_url = json_data.get("assets")[0].get("browser_download_url")

    # 固定使用镜像站下载地址
    download_url = "https://moon.ohmydeck.net"

    logger.info(download_url)

    file_path = f"/tmp/{decky_plugin.DECKY_PLUGIN_NAME}.tag.gz"

    with urllib.request.urlopen(download_url, context=gcontext) as response, open(
        file_path, "wb"
    ) as output_file:
        output_file.write(response.read())
        output_file.close()

    return file_path


def get_version():
    return f"{decky_plugin.DECKY_PLUGIN_VERSION}"


def get_latest_version():
    gcontext = ssl.SSLContext()

    response = urllib.request.urlopen(API_URL, context=gcontext)
    json_data = json.load(response)

    tag = json_data.get("tag_name")
    # if tag is a v* tag, remove the v
    if tag.startswith("v"):
        tag = tag[1:]
    else:
        return ""
    return tag
