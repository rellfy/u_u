#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Attribute {
    pub name: String,
    pub value: Option<String>,
}

impl Attribute {
    pub fn new(name: &str, value: Option<&str>) -> Attribute {
        Attribute {
            name: name.to_string(),
            value: if value.is_some() {
                Some(value.unwrap().to_string())
            } else {
                None
            }
        }
    }
}