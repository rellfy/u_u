use crate::env;

pub fn log(msg: &str) {
    unsafe {
        env::console_log(msg.as_ptr(), msg.len());
    }
}

pub fn get_uuidv4() -> String {
    let mut string;

    unsafe {
        let size = env::uuidV4();
        let data = &env::get_buffer()[0..size];
        string = String::from_utf8_lossy(data).parse().unwrap();
    }

    string
}