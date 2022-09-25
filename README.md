# React-Frontend Plugin Template [![Chat](https://img.shields.io/badge/chat-on%20discord-7289da.svg)](https://discord.gg/ZU74G2NJzk)


Reference example for using [decky-frontend-lib](https://github.com/SteamDeckHomebrew/decky-frontend-lib) in a [decky-loader](https://github.com/SteamDeckHomebrew/deckly-loader) plugin.

If you want to get in contact with the developers, we can be found in the
**[SteamDeckHomebrew Discord](https://discord.gg/ZU74G2NJzk)**.

## Developers

### Dependencies

This template relies on the user having `pnpm` installed on their system.  
This can be downloaded from `npm` itself which is recommended. 

#### Linux

```bash
sudo npm i -g pnpm
```

### Making your own plugin

1. You can fork this repo or utilize the "Use this template" button on Github.
2. In your local fork/own plugin-repository run these commands:
   1. ``pnpm i``
   2. ``pnpm run build``
   - These setup pnpm and build the frontend code for testing.
3. Consult the [decky-frontend-lib](https://github.com/SteamDeckHomebrew/decky-frontend-lib) repository for ways to accomplish your tasks.
   - Documentation and examples are still rough, 
   - While decky-loader primarily targets Steam Deck hardware so keep this in mind when developing your plugin.
4. If you want an all encompassing demonstration of decky-frontend-lib's capabilites check out [decky-playground](https://github.com/SteamDeckHomebrew/decky-playground). It shows off almost all of decky-frontend-lib's features.

#### Other important information

Everytime you change the frontend code (`index.tsx` etc) you will need to rebuild using the commands from step 2 above or the build task if you're using vscode or a derivative.

Note: If you are receiving build errors due to an out of date library, you should run this command inside of your repository:

```bash
pnpm update decky-frontend-lib --latest
```

### Backend support

If you are developing with a backend for a plugin and would like to submit it to the [decky-plugin-database](https://github.com/SteamDeckHomebrew/decky-plugin-database) you will need to have all backend code located in ``backend/src``, with backend being located in the root of your git repository.
When building your plugin, the source code will be built and any finished binary or binaries will be output to ``backend/out`` (which is created during CI.)
If your buildscript, makefile or any other build method does not place the binary files in the ``backend/out`` directory they will not be properly picked up during CI and your plugin will not have the required binaries included for distribution.

Example:  
In our makefile used to demonstrate the CI process of building and distributing a plugin backend, note that the makefile explicitly creates the `out` folder (``backend/out``) and then compiles the binary into that folder. Here's the relevant snippet.

```make
hello:
	mkdir -p ./out
	gcc -o ./out/hello ./src/main.c
```

The CI does create the `out` folder itself but we recommend creating it yourself if possible during your build process to ensure the build process goes smoothly.

The out folder is not sent to the final plugin, but is then put into a ``bin`` folder which is found at the root of the plugin's directory.  
More information on the bin folder can be found below in the distribution section below.

### Distribution

We recommend following the instructions found in the [decky-plugin-database](https://github.com/SteamDeckHomebrew/decky-plugin-database) on how to get your plugin up on the plugin store. This is the best way to get your plugin in front of users.
You can also choose to do distribution via a zip file containing the needed files, if that zip file is uploaded to a URL it can then be downloded and installed via decky-loader.

**NOTE: We do not currently have a method to install from a downloaded zip file in "game-mode" due to lack of a usable file-picking dialog.**

Layout of a plugin zip ready for distribution:
```
pluginname-v1.0.0.zip (version number is optional but recommended for users sake)
   |
   pluginname/ <directory>
   |  |  |
   |  |  bin/ <directory> (optional)
   |  |     |
   |  |     binary (optional)
   |  |
   |  dist/ <directory> [required]
   |      |
   |      index.js [required]
   | 
   package.json [required]
   plugin.json [required]
   main.py {required if you are using the python backend of decky-loader: serverAPI}
   README.md (optional but recommended)
   LICENSE(.md) {required in some cases, not in others}
```

Note regarding licenses: Including a license is required for the plugin store if your chosen license requires the license to be included alongside usage of source-code/binaries!

We cannot and will not distribute your plugin on the Plugin Store if it's license requires it's inclusion but you have not included a license to be re-distributed with your plugin in the root of your git repository.
