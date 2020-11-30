pub static mut BUFFER: [u8; 10_000] = [0; 10_000];

pub fn get_buffer() -> &'static [u8; 10_000] {
    unsafe {
        &BUFFER
    }
}

pub fn get_buffer_slice(size: usize) -> Vec<u8> {
    unsafe {
        (&BUFFER[0..size]).to_owned()
    }
}

pub fn get_buffer_as_string(size: usize) -> String {
    let string;
    let data = &get_buffer()[0..size];
    string = String::from_utf8_lossy(data).parse().unwrap();
    string
}

pub fn send_bytes(value: &[u8], f: unsafe extern  fn(*const u8, usize)) {
    unsafe {
        f(value.as_ptr(), value.len());
    }
}

pub fn send_bytes_return(value: &[u8], f: unsafe extern fn(*const u8, usize) -> usize) -> usize {
    unsafe {
        f(value.as_ptr(), value.len())
    }
}

pub fn send_string(string: &str, f: unsafe extern fn(*const u8, usize)) {
    let bytes = string.as_bytes();
    send_bytes(bytes, f);
}

pub fn send_string_return(string: &str, f: unsafe extern  fn(*const u8, usize) -> usize) -> usize {
    let bytes = string.as_bytes();
    send_bytes_return(bytes, f)
}

#[no_mangle]
extern "C" {
    pub fn console_log(msg: *const u8, length: usize);
    pub fn sync_elements(data: *const u8, length: usize);
    pub fn get_element_by_id(id: *const u8, length: usize) -> usize;
    pub fn uuidV4() -> usize;
}

#[no_mangle]
fn get_buffer_pointer() -> *const u8 {
    unsafe {
        BUFFER.as_ptr()
    }
}