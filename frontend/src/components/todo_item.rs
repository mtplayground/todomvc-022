use leptos::*;

use crate::api::{self, Todo, UpdateTodo};
use crate::state::TodoState;

#[component]
pub fn TodoItem(todo: Todo, state: TodoState) -> impl IntoView {
    let id = todo.id;
    let (completed, set_completed) = create_signal(todo.completed);
    let (title, set_title) = create_signal(todo.title.clone());
    let (editing, set_editing) = create_signal(false);
    let (edit_text, set_edit_text) = create_signal(todo.title.clone());

    let edit_input_ref = create_node_ref::<html::Input>();

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

    let on_dblclick = move |_| {
        set_edit_text.set(title.get());
        set_editing.set(true);
        if let Some(input) = edit_input_ref.get() {
            request_animation_frame(move || {
                let _ = input.focus();
            });
        }
    };

    let save_edit = move || {
        let text = edit_text.get().trim().to_string();
        set_editing.set(false);

        if text.is_empty() {
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
        } else if text != title.get() {
            set_title.set(text.clone());
            let set_todos = state.set_todos;
            spawn_local(async move {
                let update = UpdateTodo {
                    title: Some(text),
                    completed: None,
                };
                match api::update_todo(id, &update).await {
                    Ok(_) => {
                        if let Ok(todos) = api::fetch_todos().await {
                            set_todos.set(todos);
                        }
                    }
                    Err(e) => {
                        leptos::logging::error!("Failed to update todo: {}", e);
                    }
                }
            });
        }
    };

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        match ev.key().as_str() {
            "Enter" => save_edit(),
            "Escape" => {
                set_edit_text.set(title.get());
                set_editing.set(false);
            }
            _ => {}
        }
    };

    let on_blur = move |_| {
        if editing.get() {
            save_edit();
        }
    };

    view! {
        <li class:completed=completed class:editing=editing>
            <div class="view">
                <input
                    class="toggle"
                    type="checkbox"
                    prop:checked=completed
                    on:change=on_toggle
                />
                <label on:dblclick=on_dblclick>{title}</label>
                <button class="destroy" on:click=on_destroy></button>
            </div>
            <input
                class="edit"
                node_ref=edit_input_ref
                prop:value=edit_text
                on:input=move |ev| {
                    set_edit_text.set(event_target_value(&ev));
                }
                on:blur=on_blur
                on:keydown=on_keydown
            />
        </li>
    }
}
