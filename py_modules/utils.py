import os
import decky_plugin
from config import logger

FONT_CONFIG = """
<?xml version="1.0"?>
<!-- ToMoon -->
<!DOCTYPE fontconfig SYSTEM "fonts.dtd">
<fontconfig>
  <alias>
    <family>sans-serif</family>
    <prefer>
      <family>Noto Sans CJK SC</family>
      <family>Noto Sans CJK TC</family>
      <family>Noto Sans CJK JP</family>
    </prefer>
  </alias>
  <alias>
    <family>serif</family>
    <prefer>
      <family>Noto Serif CJK SC</family>
      <family>Noto Serif CJK TC</family>
      <family>Noto Serif CJK JP</family>
    </prefer>
  </alias>
  <alias>
    <family>monospace</family>
    <prefer>
      <family>Noto Sans Mono CJK SC</family>
      <family>Noto Sans Mono CJK TC</family>
      <family>Noto Sans Mono CJK JP</family>
    </prefer>
  </alias>
</fontconfig>
"""
FONT_CONF_DIR = f"{decky_plugin.DECKY_USER_HOME}/.config/fontconfig/conf.d"
FONT_CONF_FILE = f"{FONT_CONF_DIR}/75-noto-cjk.conf"


def write_font_config():
    if not os.path.exists(FONT_CONF_DIR):
        logger.info(f"Creating fontconfig directory: {FONT_CONF_DIR}")
        os.makedirs(FONT_CONF_DIR)
    if not os.path.exists(FONT_CONF_FILE):
        logger.info(f"Creating fontconfig file: {FONT_CONF_FILE}")
        with open(FONT_CONF_FILE, "w") as f:
            f.write(FONT_CONFIG)
            f.close()

    user = decky_plugin.DECKY_USER
    # change fontconfig owner
    os.system(f"chown -R {user}:{user} {FONT_CONF_DIR}")


def remove_font_config():
    if os.path.exists(FONT_CONF_FILE):
        # read fontconfig file, if contains '<!-- ToMoon -->' then remove it
        with open(FONT_CONF_FILE, "r") as f:
            content = f.read()
            f.close()
        if "<!-- ToMoon -->" in content:
            logger.info(f"Removing fontconfig file: {FONT_CONF_FILE}")
            os.remove(FONT_CONF_FILE)
