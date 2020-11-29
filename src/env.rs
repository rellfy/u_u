pub static mut BUFFER: [u8; 10_000] = [0; 10_000];

pub fn get_buffer() -> &'static [u8; 10_000] {
    unsafe {
        &BUFFER
    }
}

#[no_mangle]
extern "C" {
    pub fn console_log(msg: *const u8, length: usize);
    pub fn uuidV4() -> usize;
}

#[no_mangle]
fn get_buffer_pointer() -> *const u8 {
    unsafe {
        BUFFER.as_ptr()
    }
}