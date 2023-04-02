@ECHO OFF
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir .\target\wasm32-unknown-unknown\release-min --target web .\target\wasm32-unknown-unknown\release\learn_bevy_tutorial.wasm