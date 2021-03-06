use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use crate::{
    events::*,
    dom::*,
    util,
    env,
};

static mut ROOT: Option<Element> = None;

#[derive(Debug, Serialize)]
pub struct Element {
    uuid: String,
    parent: Option<String>,
    name: String,
    text: String,
    attributes: HashMap<String, Attribute>,
    #[serde(skip_serializing, skip_deserializing)]
    elements: Vec<Element>,
}

impl Element {
    pub fn new(name: &str) -> Element {
        Element {
            uuid: util::get_uuidv4(),
            parent: None,
            name: name.to_string(),
            text: String::new(),
            attributes: HashMap::new(),
            elements: Vec::new(),
        }
    }

    pub fn create(name: &str) -> &mut Element {
        Element::root().add_element(name)
    }

    pub fn root() -> &'static mut Element {
        unsafe {
            if ROOT.is_none() {
                ROOT = Some(Element::get_document_element_by_id("root").unwrap());
            }

            ROOT.as_mut().unwrap()
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
        let uuid = env::request_string("getElementById", id.as_bytes());

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
        if self.name != "text" {
            let existing_text = self.get_element_by_name("text");

            if existing_text.is_some() {
                let element = existing_text.unwrap();
                element.set_text(text);
                element.sync();
            } else {
                self.add_text(text);
            }

            return;
        }

        self.text = text.to_owned();
        self.sync();
    }

    pub fn add_text(&mut self, text: &str) {
        let element = self.add_element("text");
        element.set_text(text);
        element.sync();
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
        let mut element = Element::new(name);
        let uuid = element.get_uuid().to_owned();
        element.set_parent(self.uuid.clone());
        self.elements.push(element);
        self.get_element_by_uuid(uuid.as_str()).unwrap()
    }

    pub fn add_event_listener(
        &mut self,
        event: &str,
        callback: Box<dyn FnMut(Event) + 'static>
    ) {
        let event_uuid = util::get_uuidv4();

        event::get_event_listeners().insert(
            event_uuid.clone(),
            callback
        );

        let data  = format!(
            r#"{{
                "element_uuid": "{}",
                "event_uuid": "{}",
                "event": "{}"
            }}"#,
            self.uuid,
            event_uuid,
            event
        );

        env::send_bytes("addEventListener", data.as_bytes());
    }

    fn sync(&self) {
        let json = serde_json::to_string(self).unwrap();
        env::send_bytes("syncElements", json.as_bytes());
    }
}