"use strict";

window.u_u = new (class {
    constructor() {
        this.root = null;
        this.elements = {};
    }

    get instance() {
        return this.root.instance;
    }

    /**
     * @param {string} string
     */
    stringToUint8Array(string) {
        const array = new Uint8Array(string.length);
        for (let i in string) {
            array[i] = string.charCodeAt(i);
        }
        return array;
    }

    sendUint8Array(array) {
        const bufferPointer = this.instance.exports.get_buffer_pointer();
        const u8 = new Uint8Array(this.instance.exports.memory.buffer, bufferPointer, array.length);
        for (let i = 0; i < array.length; i++) {
            u8[i] = array[i];
        }
    }

    envUuidV4() {
        const uuid = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(
            /[xy]/g,
            (c) => {
                const r = Math.random() * 16 | 0;
                const v = c == "x" ? r : (r & 0x3 | 0x8);
                return v.toString(16);
            }
        );
        let array = this.stringToUint8Array(uuid);
        this.sendUint8Array(array);
        return array.length;
    }

    /**
     * Configures the WASM importObject variable on top of an existing object.
     * @param {Object} importObject The WASM importObject. Could be an empty object "{ }".
     * @param {{ instance }} root Object containing an "instance" field.
     */
    configureImportObject(importObject, root) {
        this.root = root;
        importObject.env = importObject.env ?? {};
        importObject.env.console_log = (ptr, length) => {
            const bytes = new Uint8Array(this.instance.exports.memory.buffer, ptr, length);
            console.log(new TextDecoder().decode(bytes));
        };
        importObject.env.uuidV4 = this.envUuidV4.bind(this);
    }

    /**
     * Applies changes to the DOM according to streaming WASM element data.
     * @param data The update string in the format of
     * {uuid}
     * \n
     * {element_name}
     * \n
     * {element_text}
     * \n
     * {attribute_name}
     * \n
     * {attribute_value}
     * \n
     * {child_uuid}
     *     ...
     *     \n
     *     ...
     * \0
     * ...
     */
    syncElements(data) {

    }
});