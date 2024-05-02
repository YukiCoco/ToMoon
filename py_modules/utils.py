import os
import decky_plugin
from config import logger

def write_font_config():
    user_home = decky_plugin.DECKY_USER_HOME
    font_config = """
<?xml version="1.0"?>
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
    consf_dir = f"{user_home}/.config/fontconfig/conf.d"
    if not os.path.exists(consf_dir):
        logger.info(f"Creating fontconfig directory: {consf_dir}")
        os.makedirs(consf_dir)
    conf_path = f"{consf_dir}/75-noto-cjk.conf"
    if not os.path.exists(conf_path):
        logger.info(f"Creating fontconfig file: {conf_path}")
        with open(conf_path, "w") as f:
            f.write(font_config)
            f.close()