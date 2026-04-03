use leptos::*;

use crate::api;
use crate::state::{Filter, TodoState};

#[component]
pub fn Footer(state: TodoState) -> impl IntoView {
    let on_clear_completed = move |_| {
        let set_todos = state.set_todos;
        spawn_local(async move {
            match api::delete_completed().await {
                Ok(_) => {
                    if let Ok(todos) = api::fetch_todos().await {
                        set_todos.set(todos);
                    }
                }
                Err(e) => leptos::logging::error!("Failed to clear completed: {}", e),
            }
        });
    };

    view! {
        <footer class="footer" style:display=move || {
            if state.todos.get().is_empty() { "none" } else { "" }
        }>
            <span class="todo-count">
                <strong>{move || state.active_count()}</strong>
                {move || if state.active_count() == 1 { " item left" } else { " items left" }}
            </span>
            <ul class="filters">
                <li>
                    <a
                        href="#/"
                        class:selected=move || state.filter.get() == Filter::All
                    >"All"</a>
                </li>
                <li>
                    <a
                        href="#/active"
                        class:selected=move || state.filter.get() == Filter::Active
                    >"Active"</a>
                </li>
                <li>
                    <a
                        href="#/completed"
                        class:selected=move || state.filter.get() == Filter::Completed
                    >"Completed"</a>
                </li>
            </ul>
            {move || {
                if state.completed_count() > 0 {
                    Some(view! {
                        <button class="clear-completed" on:click=on_clear_completed>
                            "Clear completed"
                        </button>
                    })
                } else {
                    None
                }
            }}
        </footer>
    }
}
