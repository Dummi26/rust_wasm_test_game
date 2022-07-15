use wasm_bindgen::JsCast;
use web_sys::{Event, MouseEvent};
use std::{sync::mpsc::Sender, rc::Rc};

use crate::renderer::{ImpInfo, Interactions};

pub fn down(event: Event, sender: Sender<Interactions>) {
    let event: MouseEvent = event.dyn_into().expect("Wrong event type.");
    sender.send(Interactions::MouseDown { button: event.button(), x: event.client_x(), y: event.client_y(), });
}
pub fn moved(event: Event, sender: Sender<Interactions>) {
    let event: MouseEvent = event.dyn_into().expect("Wrong event type.");
    sender.send(Interactions::MouseMove { button: event.button(), x: event.client_x(), y: event.client_y(), });
}