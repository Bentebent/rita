ROOT_DIR := $(subst \,/,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

build-debug:
	cargo build --verbose

build-release:
	cargo build --release --verbose

build-wasm:
	wasm-pack build --target web --release --verbose

wasm-chrome:
	.\chrome.bat "$(ROOT_DIR)index.html"