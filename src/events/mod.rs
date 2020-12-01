pub mod mouse_event;

pub enum Event {
    MouseEvent(mouse_event::Data)
}