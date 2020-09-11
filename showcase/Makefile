SHELL := /bin/bash

debug:
	cargo build --target=wasm32-unknown-emscripten
	mkdir -p ../docs
	cp ../target/wasm32-unknown-emscripten/debug/deps/raylib_showcase.data ../docs/raylib_showcase.data
	cp ../target/wasm32-unknown-emscripten/debug/raylib_showcase.wasm ../docs/raylib_showcase.wasm
	cp ../target/wasm32-unknown-emscripten/debug/raylib_showcase.wasm.map ../docs/raylib_showcase.wasm.map
	cp ../target/wasm32-unknown-emscripten/debug/raylib-showcase.d ../docs/raylib-showcase.d
	cp ../target/wasm32-unknown-emscripten/debug/raylib-showcase.js ../docs/raylib-showcase.js
