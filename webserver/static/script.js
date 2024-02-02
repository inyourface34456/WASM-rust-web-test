import init, { greet } from "./pkg/wasm_pack.js";
init().then(() => {
    greet("WebAssembly", 20);
});