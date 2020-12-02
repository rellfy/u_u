use serde::Deserialize;

pub mod mouse_event;

#[derive(Debug, Deserialize)]
pub enum Event {
    MouseEvent(mouse_event::Data)
}