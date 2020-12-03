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
        header_2.set_attribute("style", Some("color: grey; user-select: none;"));
        header_2.set_text("u_u - click me!");
    }

    // change something; add a click handler
    {
        let header  = Element::root().get_element_by_name("h1").unwrap();
        // Add an exclamation mark to h1 text.
        header.set_text("Hello, world!");
        // Add a click event listener to h2.
        let header2 = header.get_element_by_name("h4").unwrap();
        let mut counter = 0;
        let mut counter_shift = 0;
        header2.add_event_listener("click", Box::new(move |event| {
            let Event::MouseEvent(data) = event;
            counter  = counter + 1;
            if data.shiftKey {
                counter_shift = counter_shift + 1;
            }
            let header2  = Element::root().get_element_by_name("h4").unwrap();
            header2.set_text(
                format!(
                    "u_u clicked {} times - {} times holding shift",
                    counter,
                    counter_shift
                ).as_str()
            );
        }));
    }
}