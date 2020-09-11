SHELL := /bin/bash

debug:
	cargo build --target=wasm32-unknown-emscripten
	mkdir -p site
	cp ../target/wasm32-unknown-emscripten/debug/deps/raylib_showcase.data site/raylib_showcase.data
	cp ../target/wasm32-unknown-emscripten/debug/raylib_showcase.wasm site/raylib_showcase.wasm
	cp ../target/wasm32-unknown-emscripten/debug/raylib_showcase.wasm.map site/raylib_showcase.wasm.map
	cp ../target/wasm32-unknown-emscripten/debug/raylib-showcase.d site/raylib-showcase.d
	cp ../target/wasm32-unknown-emscripten/debug/raylib-showcase.js site/raylib-showcase.js
