use leptos::*;
use wasm_bindgen::prelude::*;

use crate::state::{Filter, TodoState};

pub fn init_router(state: TodoState) {
    // Set initial filter from current hash
    let hash = get_hash();
    state.set_filter.set(Filter::from_hash(&hash));

    // Listen for hashchange events
    let closure = Closure::<dyn Fn()>::new(move || {
        let hash = get_hash();
        state.set_filter.set(Filter::from_hash(&hash));
    });

    if let Some(window) = web_sys::window() {
        let _ = window.add_event_listener_with_callback(
            "hashchange",
            closure.as_ref().unchecked_ref(),
        );
    }

    // Prevent the closure from being dropped
    closure.forget();
}

fn get_hash() -> String {
    web_sys::window()
        .and_then(|w| w.location().hash().ok())
        .unwrap_or_default()
}
