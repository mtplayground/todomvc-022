pub mod api;
pub mod components;
pub mod router;
pub mod state;

use leptos::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::components::footer::Footer;
use crate::components::header::Header;
use crate::components::todo_list::TodoList;
use crate::state::TodoState;

#[wasm_bindgen(start)]
pub fn main() {
    mount_to_body(App);
}

#[component]
pub fn App() -> impl IntoView {
    let state = TodoState::new();
    router::init_router(state);

    // Load todos on mount
    let set_todos = state.set_todos;
    spawn_local(async move {
        if let Ok(todos) = api::fetch_todos().await {
            set_todos.set(todos);
        }
    });

    view! {
        <section class="todoapp">
            <Header state=state />
            <TodoList state=state />
            <Footer state=state />
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos"</p>
            <p>"Part of " <a href="http://todomvc.com">"TodoMVC"</a></p>
        </footer>
    }
}
