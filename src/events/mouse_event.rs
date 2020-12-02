use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Data {
    pub altKey: bool,
    pub ctrlKey: bool,
    pub shiftKey: bool,
    pub metaKey: bool,
    pub which: i32,
    pub button: i32,
    pub clientX: i32,
    pub clientY: i32,
    pub movementX: i32,
    pub movementY: i32,
    pub screenX: i32,
    pub screenY: i32,
    pub pageX: i32,
    pub pageY: i32,
    pub offsetX: i32,
    pub offsetY: i32,
}