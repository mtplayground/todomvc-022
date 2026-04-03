pub mod api;
pub mod state;

use leptos::*;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <section class="todoapp">
            <header class="header">
                <h1>"todos"</h1>
            </header>
        </section>
        <footer class="info">
            <p>"Double-click to edit a todo"</p>
            <p>"Created with Leptos"</p>
            <p>"Part of " <a href="http://todomvc.com">"TodoMVC"</a></p>
        </footer>
    }
}
