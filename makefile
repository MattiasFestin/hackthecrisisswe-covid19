SHELL := /bin/bash

all: install build

########################################################

install: install_nvm install_rustup install_cargo_watch install_api install_ui

install_nvm:
	@if [ -z "${NVM_DIR}" ]; then\
		curl --proto '=https' --tlsv1.2 -sSf https://raw.githubusercontent.com/nvm-sh/nvm/v0.35.0/install.sh | sh; \
		nvm use; \
	fi

install_rustup:
	@if [ -z "${NVM_DIR}" ]; then\
		curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh; \
		rustup default nightly; \
	fi

install_api: build_api

install_cargo_watch:
	cargo install cargo-watch --force

install_ui:
	pushd ui && \
	npm i && \
	popd

########################################################

build: build_api build_ui

build_api: # TODO: Docker
	pushd api && \
	cargo build && \
	popd

build_ui:  # TODO: Docker
	pushd ui && \
	npm run build && \
	popd

########################################################

test: test_api test_ui

test_api:
	pushd api && \
	cargo test && \
	popd

test_ui:
	pushd ui && \
	npm test && \
	popd


########################################################


clean: clean_containers clean_api clean_ui

clean_containers:
	docker-compose -f ./docker-compose.dev.yml -f ./docker-compose.yml stop && \
	docker-compose -f ./docker-compose.dev.yml -f ./docker-compose.yml rm

clean_api:
	pushd api && \
	cargo clean && \
	popd

clean_ui:
	pushd ui && \
	rm -rf ./node_modules ./static/* && \
	popd

########################################################

run:
	npx concurrently "make run_ui" "make run_containers" "make run_api"

run_ui:
	pushd ui && \
	NODE_ENV=development npm start && \
	popd

run_api:
	pushd ui && \
	DEBUG=1 cargo watch -x run && \
	popd

run_containers:
	docker-compose -f ./docker-compose.dev.yml -f ./docker-compose.yml up -d
