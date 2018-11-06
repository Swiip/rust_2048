SHELL := /bin/bash

all:
	cargo build --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/debug/lib_2048.wasm web/wasm/lib_2048.wasm
