use u_u;
use u_u::dom::*;

fn main() {
    // get root
    let mut root = Element::get_document_element_by_id("root").unwrap();
    u_u::log(&*format!("root uuid: {}", root.get_uuid()));

    // add elements
    {
        root.set_attribute("id", Some("root"));
        root.add_element("p");
        let header = root.add_element("h1");
        let header_text = header.add_element("text");
        header_text.set_text("Hello, world");

        let header_2 = header.add_element("h4");
        header_2.set_attribute("style", Some("color: grey"));
        let header_2_text = header_2.add_element("text");
        header_2_text.set_text("u_u");
    }

    // change something
    {
        let header_text  = root.get_element_by_name("text").unwrap();
        header_text.set_text("Hello, world!");
    }
}