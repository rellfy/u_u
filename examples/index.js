"use strict";

(async () => {
    const path = "../target/wasm32-unknown-unknown/release/examples/hello_world.wasm";
    const importObject = {};

    window.u_u.configureImportObject(importObject, this);

    const { instance } = await WebAssembly.instantiateStreaming(
        fetch(path),
        importObject
    );

    this.instance = instance;
    instance.exports.main();
})();