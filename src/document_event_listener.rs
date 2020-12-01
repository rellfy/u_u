use std::collections::HashMap;
use crate::events::*;

pub struct DocumentEventListener {
    events: HashMap<String, Event>,
}