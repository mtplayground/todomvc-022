use leptos::*;

use crate::api::Todo;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Filter {
    All,
    Active,
    Completed,
}

impl Filter {
    pub fn from_hash(hash: &str) -> Self {
        match hash {
            "#/active" => Filter::Active,
            "#/completed" => Filter::Completed,
            _ => Filter::All,
        }
    }

    pub fn matches(&self, todo: &Todo) -> bool {
        match self {
            Filter::All => true,
            Filter::Active => !todo.completed,
            Filter::Completed => todo.completed,
        }
    }
}

#[derive(Clone, Copy)]
pub struct TodoState {
    pub todos: ReadSignal<Vec<Todo>>,
    pub set_todos: WriteSignal<Vec<Todo>>,
    pub filter: ReadSignal<Filter>,
    pub set_filter: WriteSignal<Filter>,
}

impl TodoState {
    pub fn new() -> Self {
        let (todos, set_todos) = create_signal(Vec::<Todo>::new());
        let (filter, set_filter) = create_signal(Filter::All);
        TodoState {
            todos,
            set_todos,
            filter,
            set_filter,
        }
    }

    pub fn active_count(&self) -> usize {
        self.todos
            .get()
            .iter()
            .filter(|t| !t.completed)
            .count()
    }

    pub fn completed_count(&self) -> usize {
        self.todos
            .get()
            .iter()
            .filter(|t| t.completed)
            .count()
    }

    pub fn all_completed(&self) -> bool {
        let todos = self.todos.get();
        !todos.is_empty() && todos.iter().all(|t| t.completed)
    }

    pub fn filtered_todos(&self) -> Vec<Todo> {
        let filter = self.filter.get();
        self.todos
            .get()
            .iter()
            .filter(|t| filter.matches(t))
            .cloned()
            .collect()
    }
}
