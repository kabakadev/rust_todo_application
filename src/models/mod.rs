pub mod priority;
pub mod todo;

// Re-export types so callers can do `use rust_todo_core::{Todo, ...};`
pub use priority::Priority;
pub use todo::{CreateTodo, Todo, UpdateTodo};
