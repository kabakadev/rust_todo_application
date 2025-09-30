// Root library entrypoint, so other crates (src-tauri) can reuse your models.
pub mod models;

// (Optional) Re-export for shorter import paths in src-tauri:
pub use models::{CreateTodo, Priority, Todo, UpdateTodo};
