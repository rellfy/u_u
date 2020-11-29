use u_u;
use u_u::dom::*;

fn main() {
    u_u::log("Hello, world!");
    let mut root = Element::get_by_id("root").unwrap();
    u_u::log(&*format!("root uuid: {}", root.get_uuid()));

    root.add_attribute("id", Some("root"));
    root.add_element("div");
    root.add_element("p");

    u_u::log(format!("{:?}", root).as_str());
    u_u::log(format!("data string: {}", root.generate_data_string()).as_str());
    u_u::sync_elements(root);

}