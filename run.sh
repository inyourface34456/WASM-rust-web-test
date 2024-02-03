cd wasm_pack
wasm-pack build --target web
if [ $? != 0 ]; then 
exit 
fi
cd pkg
wasm-opt -Oz wasm_pack_bg.wasm -o wasm_pack_bg.wasm
cd ../..
cp wasm_pack/pkg/* webserver/static/pkg
cd webserver
cargo run