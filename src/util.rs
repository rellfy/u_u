use crate::env;

pub fn log(msg: &str) {
    env::send_string(msg, env::console_log);
}

pub fn get_uuidv4() -> String {
    let mut string;

    unsafe {
        let size = env::uuidV4();
        string = env::get_buffer_as_string(size);
    }

    string
}