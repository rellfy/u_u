"use strict";

window.u_u = new (class {
    constructor() { }

    configureImportObject(importObject, root) {
        importObject.env = importObject.env ?? {};
        importObject.env.console_log = (ptr, length) => {
            const bytes = new Uint8Array(root.instance.exports.memory.buffer, ptr, length);
            console.log(new TextDecoder().decode(bytes));
        };
    }
});