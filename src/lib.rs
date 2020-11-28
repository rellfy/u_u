#[no_mangle]
extern "C" {
    fn console_log(msg: *const u8, length: usize);
}

pub fn log(msg: &str) {
    unsafe {
        console_log(msg.as_ptr(), msg.len());
    }
}