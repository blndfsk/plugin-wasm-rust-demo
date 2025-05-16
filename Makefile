.PHONY: build clean

plugin:
	

build:
	cargo build --target wasm32-wasip1 --release
clean:
	cargo clean

