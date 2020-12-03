use std::collections::HashMap;
use serde::Deserialize;
use crate::{
    events::Event,
    dom::Element
};

pub type EventListeners = HashMap<String, Box<dyn FnMut(Event) + 'static>>;

const BUFFER_SIZE: usize = 64_000;
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut EVENT_LISTENERS: Option<EventListeners> = None;

#[derive(Debug, Deserialize)]
struct EventTrigger {
    uuid: String,
    event: Event,
}

pub fn get_event_listeners<'evli>() -> &'evli mut EventListeners {
    unsafe {
        if EVENT_LISTENERS.is_none() {
            EVENT_LISTENERS = Some(HashMap::new());
        }

        EVENT_LISTENERS.as_mut().unwrap()
    }
}

fn get_buffer_slice(size: usize) -> Vec<u8> {
    unsafe {
        (&BUFFER[0..size]).to_owned()
    }
}

#[no_mangle]
fn get_element_event_buffer_pointer() -> *const u8 {
    unsafe {
        BUFFER.as_ptr()
    }
}

#[no_mangle]
fn element_trigger_event(size: usize) {
    let bytes = get_buffer_slice(size);
    let string = String::from_utf8(bytes).unwrap();
    let trigger: EventTrigger = serde_json::from_str(string.as_str()).unwrap();
    let handler;
    unsafe {
        handler = EVENT_LISTENERS.as_mut().unwrap().get_mut(trigger.uuid.as_str());
    }
    if handler.is_none() {
        return;
    }
    let callback = handler.unwrap();
    // convert data to Event & pass in callback.
    callback(trigger.event);
}