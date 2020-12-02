const BUFFER_SIZE: usize = 128_000;
static mut BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];

pub fn get_buffer() -> &'static [u8; BUFFER_SIZE] {
    unsafe {
        &BUFFER
    }
}

pub fn get_buffer_slice(size: usize) -> Vec<u8> {
    unsafe {
        (&BUFFER[0..size]).to_owned()
    }
}

pub fn get_buffer_slice_as_string(size: usize) -> String {
    let data = get_buffer_slice(size);
    String::from_utf8(data).unwrap()
}

pub fn send_bytes(fn_name: &str, bytes: &[u8]) -> usize {
    let fn_name_bytes = fn_name.as_bytes();
    let mut data: Vec<u8> = vec![0; fn_name_bytes.len() + bytes.len() + 1];

    for i in 0..fn_name_bytes.len() {
        data[i] = fn_name_bytes[i];
    }

    for i in (fn_name_bytes.len()+1)..(fn_name_bytes.len()+1+bytes.len()) {
        data[i] = bytes[i - (fn_name_bytes.len()+1)];
    }

    unsafe {
        upload_bytes(data.as_ptr(), data.len())
    }
}

pub fn request_bytes(fn_name: &str, bytes: &[u8]) -> Vec<u8> {
    let size = send_bytes(fn_name, bytes);
    get_buffer_slice(size)
}

pub fn request_string(fn_name: &str, bytes: &[u8]) -> String {
    String::from_utf8(request_bytes(fn_name, bytes)).unwrap()
}

pub fn get_bytes(fn_name: &str) -> Vec<u8> {
    request_bytes(fn_name, &[])
}

pub fn get_string(fn_name: &str) -> String {
    String::from_utf8(get_bytes(fn_name)).unwrap()
}

#[no_mangle]
extern "C" {
    pub fn upload_bytes(data: *const u8, length: usize) -> usize;
}

#[no_mangle]
fn get_buffer_pointer() -> *const u8 {
    unsafe {
        BUFFER.as_ptr()
    }
}