Bindings were generated with rust_bindgen with rustified enums using c11

Ex for osx
bindgen raylib.h -o bindings_osx.rs --rustified-enum .+  -- --target=x86_64-apple-darwin -std=c11

web is copied directly from linux