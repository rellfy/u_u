pub struct Data {
    pub alt_key: bool,
    pub ctrl_key: bool,
    pub shift_key: bool,
    pub meta_key: bool,
    pub which: i32,
    pub button: i32,
    pub client: (i32, i32),
    pub movement: (i32, i32),
    pub screen: (i32, i32),
    pub page: (i32, i32),
    pub target: String,
}