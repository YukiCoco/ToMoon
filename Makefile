ifneq (,$(wildcard ./.env))
	include .env
	export
endif

SHELL=bash

help: ## Display list of tasks with descriptions
	@echo "+ $@"
	@fgrep -h ": ## " $(MAKEFILE_LIST) | fgrep -v fgrep | sed -e 's/\\$$//' | sed 's/-default//' | awk 'BEGIN {FS = ": ## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

vendor: ## Install project dependencies
	@echo "+ $@"
	@pnpm i

env: ## Create default .env file
	@echo "+ $@"
	@echo -e '# Makefile tools\nDECK_USER=deck\nDECK_HOST=\nDECK_PORT=22\nDECK_HOME=/home/deck\nDECK_KEY=~/.ssh/id_rsa' >> .env
	@echo -n "PLUGIN_FOLDER=" >> .env
	@jq -r .name package.json >> .env

init: ## Initialize project
	@$(MAKE) env
	@$(MAKE) vendor
	@echo -e "\n\033[1;36m Almost ready! Just a few things left to do:\033[0m\n"
	@echo -e "1. Open .env file and make sure every DECK_* variable matches your steamdeck's ip/host, user, etc"
	@echo -e "2. Run \`\033[0;36mmake copy-ssh-key\033[0m\` to copy your public ssh key to steamdeck"
	@echo -e "3. Build your code with \`\033[0;36mmake build\033[0m\` or \`\033[0;36mmake docker-build\033[0m\` to build inside a docker container"
	@echo -e "4. Deploy your plugin code to steamdeck with \`\033[0;36mmake deploy\033[0m\`"

update-frontend-lib: ## Update decky-frontend-lib
	@echo "+ $@"
	@pnpm update decky-frontend-lib --latest

download:
	@echo "+ $@"
	@$(MAKE) clean_tmp
	@$(MAKE) download_core
	@$(MAKE) download_mmdb
	@$(MAKE) download_yacd
	@$(MAKE) download_rules
	@$(MAKE) download_subconverter

clean_tmp:
	@echo "+ $@"
	@rm -rf ./tmp/*

download_core:
	@echo "+ $@"
	@mkdir -p ./tmp
	@mkdir -p ./tmp/core
	@wget -O clash.gz https://github.com/MetaCubeX/mihomo/releases/download/v1.19.1/mihomo-linux-amd64-v1.19.1.gz
	@gzip -d clash.gz -c > ./tmp/core/clash
	@rm -f clash.gz

download_mmdb:
	@echo "+ $@"
	@mkdir -p ./tmp
	@mkdir -p ./tmp/core
	@wget -O ./tmp/core/country.mmdb https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/country.mmdb
	@wget -O ./tmp/core/geosite.dat https://github.com/MetaCubeX/meta-rules-dat/releases/download/latest/geosite.dat
	@wget -O ./tmp/core/asn.mmdb https://github.com/P3TERX/GeoLite.mmdb/raw/download/GeoLite2-ASN.mmdb

download_yacd:
	@echo "+ $@"
	@mkdir -p ./tmp
	@mkdir -p ./tmp/core
	@wget -O ./tmp/yacd.zip https://github.com/MetaCubeX/yacd/archive/gh-pages.zip
	@unzip ./tmp/yacd.zip -d ./tmp
	@mv ./tmp/Yacd-meta-gh-pages ./tmp/core/web
	@rm -f ./tmp/yacd.zip

download_rules:
	@echo "+ $@"
	@bash ./assets/subconverter_rules/dl_rules.sh ./assets/subconverter_rules

download_subconverter:
	@echo "+ $@"
	@mkdir -p ./tmp/subconverter
	@mkdir -p ./tmp/subconverter_tmp
	@wget -O subconverter_linux64.tar.gz https://github.com/MetaCubeX/subconverter/releases/download/Alpha/subconverter_linux64.tar.gz
	@tar xvf subconverter_linux64.tar.gz -C ./tmp/subconverter_tmp/
	@cp ./tmp/subconverter_tmp/subconverter/subconverter ./tmp/subconverter/
	@rm -r ./tmp/subconverter_tmp
	
build-front: ## Build frontend
	@echo "+ $@"
	@pnpm run build
	@$(MAKE) build-front-sub
	@$(MAKE) copy-file

build-front-sub:
	@echo "+ $@"
	@pnpm --prefix ./tomoon-web run build
	@mkdir -p ./web/rules
	@cp -r ./tomoon-web/dist/* ./web
	@cp -r ./assets/subconverter_rules/*.list  ./web/rules
	@cp -r ./assets/subconverter_rules/ACL4SSR_Online.ini  ./web/

copy-file:
	@echo "+ $@"
	@cp -r ./tmp/core ./bin/
	@cp ./tmp/subconverter/subconverter ./bin/subconverter

build-back: ## Build backend
	@echo "+ $@"
	@make -C ./backend

build: ## Build everything
	@$(MAKE) build-front build-back

copy-ssh-key: ## Copy public ssh key to steamdeck
	@echo "+ $@"
	@ssh-copy-id -i $(DECK_KEY) $(DECK_USER)@$(DECK_HOST)

deploy-steamdeck: ## Deploy plugin build to steamdeck
	@echo "+ $@"
	@ssh $(DECK_USER)@$(DECK_HOST) -p $(DECK_PORT) -i $(DECK_KEY) \
 		'chmod -v 755 $(DECK_HOME)/homebrew/plugins/ && mkdir -p $(DECK_HOME)/homebrew/plugins/$(PLUGIN_FOLDER)'
	@rsync -azp --delete --progress -e "ssh -p $(DECK_PORT) -i $(DECK_KEY)" \
		--chmod=Du=rwx,Dg=rx,Do=rx,Fu=rwx,Fg=rx,Fo=rx \
		--exclude='.git/' \
		--exclude='.github/' \
		--exclude='.vscode/' \
		--exclude='node_modules/' \
		--exclude='.pnpm-store/' \
		--exclude='src/' \
		--exclude='yacd' . \
		--exclude='tomoon-web/' \
		--exclude='backend/' \
		--exclude='tmp/' \
		--exclude='*.log' \
		--exclude='.gitignore' . \
		--exclude='.idea' . \
		--exclude='.env' . \
		--exclude='Makefile' . \
		--exclude='usdpl/' \
		--exclude='./assets/' \
 		./ $(DECK_USER)@$(DECK_HOST):$(DECK_HOME)/homebrew/plugins/$(PLUGIN_FOLDER)/
	@ssh $(DECK_USER)@$(DECK_HOST) -p $(DECK_PORT) -i $(DECK_KEY) \
 		'chmod -v 755 $(DECK_HOME)/homebrew/plugins/'

restart-decky: ## Restart Decky on remote steamdeck
	@echo "+ $@"
	@ssh -t $(DECK_USER)@$(DECK_HOST) -p $(DECK_PORT) -i $(DECK_KEY) \
 		'sudo systemctl restart plugin_loader.service'
	@echo -e '\033[0;32m+ all is good, restarting Decky...\033[0m'

deploy: ## Deploy code to steamdeck and restart Decky
	@$(MAKE) deploy-steamdeck
	@$(MAKE) restart-decky

it: ## Build all code, deploy it to steamdeck, restart Decky
	@$(MAKE) build deploy

cleanup: ## Delete all generated files and folders
	@echo "+ $@"
	@rm -f .env
	@rm -rf ./dist
	@rm -rf ./tmp
	@rm -rf ./node_modules
	@rm -rf ./.pnpm-store
	@rm -rf ./backend/out

uninstall-plugin: ## Uninstall plugin from steamdeck, restart Decky
	@echo "+ $@"
	@ssh -t $(DECK_USER)@$(DECK_HOST) -p $(DECK_PORT) -i $(DECK_KEY) \
 		"sudo sh -c 'rm -rf $(DECK_HOME)/homebrew/plugins/$(PLUGIN_FOLDER)/ && systemctl restart plugin_loader.service'"
	@echo -e '\033[0;32m+ all is good, restarting Decky...\033[0m'

docker-rebuild-image: ## Rebuild docker image
	@echo "+ $@"
	@docker compose build --pull

docker-build: ## Build project inside docker container
	@$(MAKE) build-back
	@echo "+ $@"
	@docker run --rm -i -v $(PWD):/plugin -v $(PWD)/tmp/out:/out ghcr.io/steamdeckhomebrew/builder:latest
