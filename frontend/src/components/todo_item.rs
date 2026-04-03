use leptos::*;

use crate::api::{self, Todo, UpdateTodo};
use crate::state::TodoState;

#[component]
pub fn TodoItem(todo: Todo, state: TodoState) -> impl IntoView {
    let id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);
    let (title, _set_title) = create_signal(todo.title.clone());

    let on_toggle = move |_| {
        let new_completed = !completed.get();
        set_completed.set(new_completed);
        let set_todos = state.set_todos;
        spawn_local(async move {
            let update = UpdateTodo {
                title: None,
                completed: Some(new_completed),
            };
            match api::update_todo(id, &update).await {
                Ok(_) => {
                    if let Ok(todos) = api::fetch_todos().await {
                        set_todos.set(todos);
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Failed to toggle todo: {}", e);
                    set_completed.set(!new_completed);
                }
            }
        });
    };

    let on_destroy = move |_| {
        let set_todos = state.set_todos;
        spawn_local(async move {
            match api::delete_todo(id).await {
                Ok(_) => {
                    if let Ok(todos) = api::fetch_todos().await {
                        set_todos.set(todos);
                    }
                }
                Err(e) => {
                    leptos::logging::error!("Failed to delete todo: {}", e);
                }
            }
        });
    };

    view! {
        <li class:completed=completed>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=on_toggle
                />
                <label>{title}</label>
                <button class="destroy" on:click=on_destroy></button>
            </div>
        </li>
    }
}
