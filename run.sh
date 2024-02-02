cd wasm_pack
wasm-pack build --target web
cd ..
cp wasm_pack/pkg/* webserver/static/pkg
cd webserver
cargo run