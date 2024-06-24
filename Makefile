ROOT_DIR := $(subst \,/,$(dir $(abspath $(lastword $(MAKEFILE_LIST)))))

build-debug:
	cargo build 

build-release:
	cargo build --release 

build-wasm:
	rustup run nightly wasm-pack build --target web --release

wasm-chrome:
	.\chrome.bat