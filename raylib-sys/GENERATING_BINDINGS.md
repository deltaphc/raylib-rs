Bindings were generated with rust_bindgen with rustified enums using c11

Ex
bindgen rgui_wrapper.h -o bindings_osx.rs --rustified-enum .+ -- --target=x86_64-apple-darwin -std=c11
bindgen rgui_wrapper.h -o bindings_linux.rs --rustified-enum .+ -- --target=x86_64-unknown-linux-gnu -std=c11
Windows and Web bindings are generated from rgui.h and the extern functions are manually copied over
bindgen rgui_wrapper.h -o bindings_windows.rs --rustified-enum .+ -- --target=x86_64-pc-windows-msvc -std=c11
bindgen rgui_wrapper.h -o bindings_web.rs --rustified-enum .+ -- --target=wasm32-unknown-emscripten -std=c11
