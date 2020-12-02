use crate::env;

pub fn log(msg: &str) {
    env::send_bytes("consoleLog", msg.as_bytes());
}

pub fn get_uuidv4() -> String {
    env::get_string("generateUuidV4")
}