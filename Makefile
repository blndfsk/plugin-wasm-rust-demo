.PHONY: build clean

plugin: build
	mkdir plugin
	cp .traefik.yml plugin/
	cp LICENSE plugin/
	cp target/wasm32-wasip1/release/http-wasm-rust.wasm plugin/plugin.wasm

build:
	cargo build --target wasm32-wasip1 --release
clean:
	cargo clean

