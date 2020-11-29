use std::collections::HashMap;
use crate::dom::Attribute;
use crate::util;

#[derive(Debug)]
pub struct Element {
    uuid: String,
    parent: Option<String>,
    name: String,
    text: String,
    attributes: HashMap<String, Attribute>,
    elements: HashMap<String, Vec<Element>>,
}

impl Element {
    pub fn create(name: &str) -> Element {
        Element {
            uuid: util::get_uuidv4(),
            parent: None,
            name: name.to_string(),
            text: String::new(),
            attributes: HashMap::new(),
            elements: HashMap::new(),
        }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
    }

    pub fn set_parent(&mut self, uuid: String) {
        self.parent = Some(uuid);
    }

    pub fn add_attribute(&mut self, name: &str, value: Option<&str>) {
        if self.attributes.contains_key(name) {
            return;
        }

        self.attributes.insert(
            name.to_string(),
            Attribute::new(name, value)
        );
    }

    pub fn add_element(&mut self, name: &str) {
        if !self.elements.contains_key(name) {
            self.elements.insert(name.to_owned(), Vec::new());
        }

        let elements = self.elements.get_mut(name).unwrap();
        let mut element = Element::create(name);
        element.set_parent(self.uuid.clone());
        elements.push(element);
    }

    /**
     * Data string format:
     * {uuid}
     * \n
     * {parent uuid} | \0
     * \n
     * {element_name}
     * \n
     * {element_text}
     * \n
     * {attribute_name}
     * \n
     * {attribute_value} | \0
     * \n
     * ...
     * {child_uuid}
     *     ...
     *     \n
     *     ...
     */
    pub fn generate_data_string(&self) -> String {
        let mut data = String::new();

        // Write UUID.
        data.push_str(self.uuid.as_str());
        data.push('\n');
        // Write parent UUID.
        if self.parent.is_some() {
            data.push_str(self.parent.as_ref().unwrap().as_str())
        } else {
            data.push('\0');
        }
        data.push('\n');
        // Write name.
        data.push_str(self.name.as_str());
        data.push('\n');
        // Write text content.
        data.push_str(self.text.as_str());
        data.push('\n');
        // Write attributes.
        for (key, value) in &self.attributes {
            data.push_str(value.name.as_str());
            data.push('\n');
            if value.value.as_ref().is_some() {
                data.push_str(value.value.as_ref().unwrap().as_str());
            } else {
                data.push('\0');
            }
            data.push('\n');
        }
        // Write elements.
        for (key, value) in &self.elements {
            for i in 0..value.len() {
                data.push_str(value[i].generate_data_string().as_str());
            }
        }

        data
    }
}