use leptos::*;

use crate::api;
use crate::components::todo_item::TodoItem;
use crate::state::TodoState;

#[component]
pub fn TodoList(state: TodoState) -> impl IntoView {
    let on_toggle_all = move |_| {
        let set_todos = state.set_todos;
        spawn_local(async move {
            match api::toggle_all().await {
                Ok(todos) => set_todos.set(todos),
                Err(e) => leptos::logging::error!("Failed to toggle all: {}", e),
            }
        });
    };

    view! {
        <section class="main" style:display=move || {
            if state.todos.get().is_empty() { "none" } else { "" }
        }>
            <input
                id="toggle-all"
                class="toggle-all"
                type="checkbox"
                prop:checked=move || state.all_completed()
                on:change=on_toggle_all
            />
            <label for="toggle-all">"Mark all as complete"</label>
            <ul class="todo-list">
                <For
                    each=move || state.filtered_todos()
                    key=|todo| todo.id
                    children=move |todo| {
                        view! { <TodoItem todo=todo state=state /> }
                    }
                />
            </ul>
        </section>
    }
}
