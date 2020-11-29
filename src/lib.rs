use crate::dom::Element;

pub mod dom;
pub mod util;
mod env;

pub use util::log;

pub fn sync_elements(root: Element) {
    let data = root.generate_data_string();
    unsafe {
        env::sync_elements(data.as_ptr(), data.len());
    }
}