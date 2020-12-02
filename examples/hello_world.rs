use serde::{Serialize, Deserialize};
use serde_json::Result;
use std::collections::HashMap;
use u_u:: {
    dom::*,
    Event,
};

fn main() {
    // add elements
    {
        let header = Element::create("h1");
        header.set_text("Hello, world");

        let header_2 = header.add_element("h4");
        header_2.set_attribute("style", Some("color: grey"));
        header_2.set_text("u_u");
    }

    // change something
    {
        // header.set_text("Hello, world!");
        // let mut counter = 0;
        let header  = Element::root().get_element_by_name("h1").unwrap();

        header.add_event_listener("click", Box::new(|event| {
            let Event::MouseEvent(data) = event;
            u_u::log(format!("Clicked. Alt key down? {}", data.altKey).as_str());
            let h  = Element::root().get_element_by_name("h1").unwrap();
            let counter = 0;
            h.set_text(format!("clicked {} times", counter).as_str());
            // counter  = counter + 1;
        }));
    }
}