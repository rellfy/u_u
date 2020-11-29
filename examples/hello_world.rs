use u_u;
use u_u::dom::*;

fn main() {
    u_u::log("Hello, world!");
    let mut div = Element::create("div");
    div.add_element("div");
    div.add_element("p");

    u_u::log(format!("{:?}", div).as_str());
    u_u::log(format!("data string: {}", div.generate_data_string()).as_str());
}