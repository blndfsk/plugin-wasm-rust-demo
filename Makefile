.PHONY: build clean

plugin: clean build
	mkdir target/plugin
	cp .traefik.yml target/plugin/
	cp LICENSE target/plugin/
	cp target/wasm32-wasip1/release/http-wasm-rust.wasm target/plugin/plugin.wasm

build:
	cargo build --target wasm32-wasip1 --release

clean:
	cargo clean

