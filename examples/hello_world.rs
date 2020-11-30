use u_u;
use u_u::dom::*;

fn main() {
    // get root
    let mut root = Element::get_document_element_by_id("root").unwrap();
    u_u::log(&*format!("root uuid: {}", root.get_uuid()));

    // add elements
    {
        root.set_attribute("id", Some("root"));
        let header = root.add_element("h1");
        header.set_text("Hello, world");

        let header_2 = header.add_element("h4");
        header_2.set_attribute("style", Some("color: grey"));
        header_2.set_text("u_u");
    }

    // change something
    {
        let header  = root.get_element_by_name("h1").unwrap();
        header.set_text("Hello, world!");
    }
}