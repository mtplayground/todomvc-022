use leptos::*;

use crate::api;
use crate::state::TodoState;

#[component]
pub fn Header(state: TodoState) -> impl IntoView {
    let (input_value, set_input_value) = create_signal(String::new());

    let on_keydown = move |ev: web_sys::KeyboardEvent| {
        if ev.key() == "Enter" {
            let title = input_value.get().trim().to_string();
            if !title.is_empty() {
                set_input_value.set(String::new());
                let set_todos = state.set_todos;
                spawn_local(async move {
                    match api::create_todo(&title).await {
                        Ok(_) => {
                            if let Ok(todos) = api::fetch_todos().await {
                                set_todos.set(todos);
                            }
                        }
                        Err(e) => {
                            leptos::logging::error!("Failed to create todo: {}", e);
                        }
                    }
                });
            }
        }
    };

    view! {
        <header class="header">
            <h1>"todos"</h1>
            <input
                class="new-todo"
                placeholder="What needs to be done?"
                autofocus=true
                prop:value=input_value
                on:input=move |ev| {
                    set_input_value.set(event_target_value(&ev));
                }
                on:keydown=on_keydown
            />
        </header>
    }
}
