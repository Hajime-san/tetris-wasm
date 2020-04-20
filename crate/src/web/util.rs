use std::cell::Cell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// pub fn add_event_listener<T: web_sys::Document,U>(event_target: T, event: U, event_name: &str, f: &Box<dyn FnMut()>) {
//     let closure = Closure::wrap(Box::new(move |event: U| {
//         f();
//     }) as Box<dyn FnMut(_)>);
//     event_target.add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref());
//     closure.forget();
// }

// fn buildCbClosure(e : impl Into<web_sys::Document>) -> Closure<dyn FnMut(web_sys::KeyboardEvent)> {
//     let mut e = e.into(); // `e` is an HtmlElement.

// }
