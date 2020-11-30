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

    getTextFromPointer(pointer, length) {
        const bytes = new Uint8Array(this.instance.exports.memory.buffer, pointer, length);
        return new TextDecoder().decode(bytes);
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
        return array.length;
    }

    sendString(string) {
        const array = this.stringToUint8Array(string);
        return this.sendUint8Array(array);
    }

    generateUuidV4(send = true) {
        const uuid = "xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx".replace(
            /[xy]/g,
            (c) => {
                const r = Math.random() * 16 | 0;
                const v = c === "x" ? r : (r & 0x3 | 0x8);
                return v.toString(16);
            }
        );

        if (!send)
            return uuid;

        return this.sendString(uuid);
    }

    consoleLog(pointer, length) {
        const message = this.getTextFromPointer(pointer, length);
        console.log(message);
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
        importObject.env.get_element_by_id = this.getElementById.bind(this);
    }

    // TODO: change to generic method with enum to retrieve by any attribute & value.
    getElementById(pointer, length) {
        const id = this.getTextFromPointer(pointer, length);
        const element = document.getElementById(id);

        if (element == null)
            return this.sendString("\0");

        const uuid = this.generateUuidV4(false);
        this.elements[uuid] = element;
        return this.sendString(uuid);
    }

    validateUuid(string) {
        return string
            .match(/^[0-9a-f]{8}-[0-9a-f]{4}-[0-5][0-9a-f]{3}-[089ab][0-9a-f]{3}-[0-9a-f]{12}$/i)
            !== null;
    }

    /**
     * @param data The update string in the format:
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
    getElementDataFromWasm(pointer, length) {
        const dataString = this.getTextFromPointer(pointer, length);
        const fields = dataString.split("\n");

        let uuid = fields[0];

        const element = {
            uuid,
            parent: fields[1].length === 1 ? null : fields[1],
            name: fields[2],
            text: fields[3],
            attributes: {}
        };

        // Set attributes.
        for (let i = 4; i < fields.length; i++) {
            const hasValue = i !== fields.length - 1 && fields[i + 1] !== '\0';
            const name = fields[i];
            const value = !hasValue ? null : fields[i + 1];
            element.attributes[name] = value;
            i++;
        }

        return element;
    }

    createElement(elementData) {
        const createFunc = elementData.name !== "text" ?
            () => document.createElement(elementData.name) :
            () => document.createTextNode(elementData.text);

        const element = createFunc();

        // console.log(elementData);

        if (elementData.parent == null)
            return element;

        this.elements[elementData.parent].appendChild(element);
        return element;
    }

    /**
     * Applies changes to the DOM according to streaming WASM element data.
     */
    applyElementChanges(element) {
        // console.log("Virtual DOM", this.virtualElements);
        const uuid = element.uuid;

        if (this.elements[uuid] == null) {
            this.virtualElements[uuid] = {};
            this.elements[uuid] = this.createElement(element);
        }

        for (let key in this.virtualElements[uuid]) {
            if (this.virtualElements[uuid][key] === element[key])
                continue;

            // Apply changes to the DOM.
            switch (key) {
                case "uuid":
                case "parent":
                    break;
                case "text":
                    this.elements[uuid].nodeValue = element[key];
                    break;
                case "attributes":
                    let names = [];

                    // Add new attributes.
                    for (const name in element[key]) {
                        if (name === "")
                            continue;

                        names.push(name);
                        const value = element[key][name] ?? "";
                        const attribute = document.createAttribute(name);
                        attribute.value = value;

                        this.elements[uuid].setAttributeNode(attribute);
                    }

                    // Remove old attributes.
                    for (const name in this.virtualElements[uuid][key]) {
                        if (name == "")
                            continue;

                        if (names.includes(name))
                            continue;

                        this.elements[uuid].removeAttribute(name);
                    }
                    break;
            }
        }

        // console.log("Set element", element);
        this.virtualElements[uuid] = element;

        // console.log(this.virtualElements);
        // console.log(this.elements);
    }

    /**
     * Fetch element data from WASM and apply changes to the DOM.
     */
    syncElements(pointer, length) {
        const element = this.getElementDataFromWasm(pointer, length);
        this.applyElementChanges(element);
    }
});