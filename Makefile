USE_SUDO := $(shell which docker >/dev/null && docker ps 2>&1 | grep -q "permission denied" && echo sudo)
DOCKER := $(if $(USE_SUDO), sudo docker, docker)
DIRNAME := $(notdir $(CURDIR))

build: check
	$(DOCKER) build . --tag $(DIRNAME)

check:
	cargo fmt
	cargo clippy --all-targets -- -D warnings
