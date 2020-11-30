use std::collections::HashMap;
use crate::{
    dom::*,
    util,
    env,
};

#[derive(Debug)]
pub struct Element {
    uuid: String,
    parent: Option<String>,
    name: String,
    text: String,
    attributes: HashMap<String, Attribute>,
    elements: Vec<Element>,
}

impl Element {
    pub fn create(name: &str) -> Element {
        Element {
            uuid: util::get_uuidv4(),
            parent: None,
            name: name.to_string(),
            text: String::new(),
            attributes: HashMap::new(),
            elements: Vec::new(),
        }
    }

    // TODO: write function to retrieve data string from JS
    pub fn from_uuid(uuid: String) -> Element {
        Element {
            uuid,
            parent: None,
            name: "div".to_owned(),
            text: String::new(),
            attributes: HashMap::new(),
            elements: Vec::new(),
        }
    }

    pub fn get_element_by_uuid<'a>(&'a mut self, uuid: &str) -> Option<&'a mut Element> {
        if self.get_uuid() == uuid {
            Some(self)
        } else {
            self.elements
                .iter_mut()
                .filter_map(|x| x.get_element_by_uuid(uuid)).next()
        }
    }

    pub fn get_element_by_id(&mut self, id: &str) -> Option<&mut Element> {
        self.get_element_by_attribute("id", id)
    }

    pub fn get_element_by_attribute<'a>(
        &'a mut self,
        name: &str,
        value: &str
    ) -> Option<&'a mut Element>
    {
        let attribute = self.attributes.get(name);

        if attribute.is_some() && attribute.unwrap().value.is_some() &&
            attribute.unwrap().value.as_ref().unwrap().as_str() == value {
            Some(self)
        } else {
            self.elements
                .iter_mut()
                .filter_map(|x| x.get_element_by_id(value)).next()
        }
    }

    pub fn get_element_by_name<'a>(&'a mut self, name: &str) -> Option<&'a mut Element> {
        if self.name == name {
            Some(self)
        } else {
            self.elements
                .iter_mut()
                .filter_map(|x| x.get_element_by_name(name)).next()
        }
    }

    pub fn get_document_element_by_id(id: &str) -> Option<Element> {
        let uuid;
        let size = env::send_string_return(id, env::get_element_by_id);
        uuid = env::get_buffer_as_string(size);

        if uuid.len() == 1 {
            return None;
        }

        let element = Element::from_uuid(uuid);
        element.sync();
        Some(element)
    }

    pub fn get_uuid(&self) -> &str {
        self.uuid.as_str()
    }

    pub fn get_name(&self) -> &str {
        self.name.as_str()
    }

    pub fn get_text(&self) -> &str {
        self.text.as_str()
    }

    pub fn set_text(&mut self, text: &str) {
        self.text = text.to_owned();
        self.sync();
    }

    pub fn set_parent(&mut self, uuid: String) {
        self.parent = Some(uuid);
        self.sync();
    }

    pub fn set_attribute(&mut self, name: &str, value: Option<&str>) {
        let attribute = self.attributes.get_mut(name);

        if attribute.is_some() {
            attribute.unwrap().value = if value.is_some() {
                Some(value.unwrap().to_owned())
            } else {
                None
            };
        } else {
            self.attributes.insert(
                name.to_string(),
                Attribute::new(name, value)
            );
        }

        self.sync();
    }

    pub fn remove_attribute(&mut self, name: &str) {
        if !self.attributes.contains_key(name) {
            return;
        }

        self.attributes.remove_entry(name);
        self.sync();
    }

    pub fn add_element(&mut self, name: &str) -> &mut Element {
        let mut element = Element::create(name);
        let uuid = element.get_uuid().to_owned();
        element.set_parent(self.uuid.clone());
        self.elements.push(element);
        self.get_element_by_uuid(uuid.as_str()).unwrap()
    }

    fn sync(&self) {
        let data = self.generate_data_string();
        unsafe {
            env::sync_elements(data.as_ptr(), data.len());
        }
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
    fn generate_data_string(&self) -> String {
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
        for (_key, value) in &self.attributes {
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
        // for i in 0..self.elements.len() {
        //     data.push_str(self.elements[i].generate_data_string().as_str());
        // }

        data
    }
}