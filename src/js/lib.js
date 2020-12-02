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

    getDataFromPointer(pointer, length) {
        return new Uint8Array(this.instance.exports.memory.buffer, pointer, length);
    }

    getTextFromPointer(pointer, length) {
        const bytes = this.getDataFromPointer(pointer, length);
        return new TextDecoder().decode(bytes);
    }

    receiveBytes(pointer, length) {
        let string = this.getTextFromPointer(pointer, length);
        let rpcSize = 0;

        for (let i = 0; i < string.length; i++) {
            rpcSize++;

            if (string[i] === "\0")
                break;
        }

        let rpcName = string.substr(0, rpcSize - 1);
        string = string.substr(rpcSize, string.length - rpcSize);

        if (string.length === 0)
            string = null;

        const object = this;
        const func = object[rpcName].bind(this);
        return func(string);
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

    /**
     * Sends a byte array to a specific buffer.
     */
    sendUint8ArrayToBuffer(array, bufferPointer) {
        const u8 = new Uint8Array(this.instance.exports.memory.buffer, bufferPointer, array.length);
        for (let i = 0; i < array.length; i++) {
            u8[i] = array[i];
        }
        return array.length;
    }

    /**
     * Sends a byte array to the main buffer.
     */
    sendUint8Array(array) {
        return this.sendUint8ArrayToBuffer(array, this.instance.exports.get_buffer_pointer());
    }

    sendString(string) {
        const array = this.stringToUint8Array(string);
        return this.sendUint8Array(array);
    }

    generateUuidV4(send = true) {
        if (send == null)
            send = true;

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

    consoleLog(message) {
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
        importObject.env.upload_bytes = this.receiveBytes.bind(this);
    }

    // TODO: change to generic method with enum to retrieve by any attribute & value.
    getElementById(id) {
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

    createElement(elementData) {
        const createFunc = elementData.name !== "text" ?
            () => document.createElement(elementData.name) :
            () => document.createTextNode(elementData.text);

        const element = createFunc();

        if (elementData.parent == null)
            return element;

        this.elements[elementData.parent].appendChild(element);
        return element;
    }

    /**
     * Applies changes to the DOM according to streaming WASM element data.
     */
    applyElementChanges(element) {
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
                        const value = element[key][name].value ?? "";
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

        this.virtualElements[uuid] = element;
    }

    /**
     * Fetch element data from WASM and apply changes to the DOM.
     */
    syncElements(json) {
        const element = JSON.parse(json);
        this.applyElementChanges(element);
    }

    addEventListener(json) {
        const { event_uuid, element_uuid, event } = JSON.parse(json);
        this.elements[element_uuid].addEventListener(event, (event) => {
            event.uuid = element_uuid;
            let stringifiedEvent = this.stringifyEvent(event);
            const eventName = event.constructor.name;
            const data = `{ "uuid": "${event_uuid}", ` +
                `"event": { "${eventName}": ${stringifiedEvent} } }`;
            let bytes = new Uint8Array(data.length);
            for (let i = 0; i < data.length; i++) {
                bytes[i] = data.charCodeAt(i);
            }
            this.sendUint8ArrayToBuffer(
                bytes,
                this.instance.exports.get_element_event_buffer_pointer()
            );
            this.instance.exports.element_trigger_event(bytes.length);
        });
    }

    stringifyEvent(event) {
        const object = {};

        for (let key in event) {
            object[key] = event[key];
        }

        return JSON.stringify(object, (key, value) => {
            if (value instanceof Node)
                return 'Node';

            if (value instanceof Window)
                return 'Window';

            return value;
        }, " ");
    }
});