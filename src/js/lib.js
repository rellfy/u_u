"use strict";

window.u_u = new (class {
    constructor() {
        this.root = null;
        this.elements = {};
        this.virtualElements = {};
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

    generateUuidV4() {
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

    consoleLog(pointer, length) {
        const bytes = new Uint8Array(this.instance.exports.memory.buffer, pointer, length);
        console.log(new TextDecoder().decode(bytes));
    };

    /**
     * Configures the WASM importObject variable on top of an existing object.
     * @param {Object} importObject The WASM importObject. Could be an empty object "{ }".
     * @param {{ instance }} root Object containing an "instance" field.
     */
    configureImportObject(importObject, root) {
        this.root = root;
        importObject.env = importObject.env ?? {};
        importObject.env.console_log = this.consoleLog.bind(this);
        importObject.env.uuidV4 = this.generateUuidV4.bind(this);
        importObject.env.sync_elements = this.syncElements.bind(this);
    }

    validateUuid(string) {
        return string
            .match(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$/i)
            !== null;
    }

    /**
     * Applies changes to the DOM according to streaming WASM element data.
     * @param data The update string in the format of
     * {uuid}
     * {parent uuid} | \0
     * \n
     * {element_name}
     * \n
     * {element_text}
     * \n
     * {attribute_name}
     * \n
     * {attribute_value} \0
     * \n
     * ...
     * {child_uuid}
     *     ...
     *     \n
     *     ...
     */
    syncElements(pointer, length) {
        const bytes = new Uint8Array(this.instance.exports.memory.buffer, pointer, length);
        const dataString = new TextDecoder().decode(bytes);
        const data = {};
        const fields = dataString.split("\n");

        let lastUuidIndex = 0;
        let lastUuid = null;

        for (let i = 0; i < fields.length; i++) {
            const field = fields[i];
            const isUuid = this.validateUuid(field);

            if (isUuid) {
                let parentUuid = fields[i + 1];
                data[field] = {
                    uuid: field,
                    parent: parentUuid.length === 1 ? null : parentUuid,
                    attributes: {}
                };
                lastUuid = field;
                lastUuidIndex = i;
                i++;
                continue;
            }

            // Get element-relative index.
            let j = i - lastUuidIndex;

            switch (j) {
                case 2:
                    data[lastUuid].name = field;
                    break;
                case 3:
                    data[lastUuid].text = field;
                    break;
                default:
                    // Attribute.
                    const hasValue = i !== fields.length - 1 && fields[i + 1] !== '\0';
                    const value = !hasValue ? null : fields[i + 1];
                    data[lastUuid].attributes[field] = value;
                    i++;
            }
        }

        console.log("Virtual DOM", data);

        // Apply virtual DOM.
        for (let uuid in data) {
            // TODO: actually compare differences & apply deltas only to DOM.
            if (this.compareVirtualElements(data[uuid], this.virtualElements[uuid]))
                continue;

            this.virtualElements[uuid] = data[uuid];

            if (this.elements[uuid] == null)
                this.elements[uuid] = document.createElement(this.virtualElements[uuid].name);

            for (let name in this.virtualElements[uuid].attributes) {
                // TODO: check why this happens.
                if (name === "")
                    continue;

                const attribute = document.createAttribute(name);
                attribute.value = this.virtualElements[uuid].attributes[name];
                this.elements[uuid].setAttributeNode(attribute);
            }

            const parentElement = this.virtualElements[uuid].parent == null ?
                document.body :
                this.elements[this.virtualElements[uuid].parent];

            parentElement.appendChild(this.elements[uuid]);
            console.log(this.elements[uuid]);
        }

    }

    compareVirtualElements(a, b) {
        if (a == null || b == null)
            return a == null && b == null;

        return a.uuid === b.uuid && a.parent === b.parent;
    }
});