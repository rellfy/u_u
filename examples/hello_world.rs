use u_u;
use u_u::dom::*;
use serde::{Serialize, Deserialize};
use serde_json::Result;

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
        let header  = Element::root().get_element_by_name("h1").unwrap();
        header.set_text("Hello, world!");
    }
}