.PHONY: help

help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

docs: #handshake.dot ## create documetation
	cd shotgun_common && cargo doc
	cd gameserver && cargo doc
	cd coward_bot && cargo doc

handshake.png: handshake.dot ## render handshake graph
	dot -T png -o handshake.png handshake.dot

kate: ## open all relevant files in kate
	kate $$(find -name "*.rs") $$(find -name "Cargo.toml") handshake.dot README.md Makefile

build: ## build everything
	cd shotgun_common && cargo build
	cd gameserver && cargo build
	cd coward_bot && cargo build
