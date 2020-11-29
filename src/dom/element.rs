use std::collections::HashMap;
use crate::dom::Attribute;
use crate::util;

#[derive(Debug)]
pub struct Element {
    uuid: String,
    name: String,
    text: String,
    attributes: HashMap<String, Attribute>,
    elements: HashMap<String, Vec<Element>>,
}

impl Element {
    pub fn create(name: &str) -> Element {
        Element {
            uuid: util::get_uuidv4(),
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
        elements.push(Element::create(name));
    }

    /**
     * Data string format:
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
    pub fn generate_data_string(&self) -> String {
        let data = String::new();

        // data.push_str(self.uuid);

        data
    }
}