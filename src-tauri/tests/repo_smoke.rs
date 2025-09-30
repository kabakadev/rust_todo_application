//! src-tauri/tests/repo_smoke.rs

use sqlx::PgPool;
// ✅ Import via the tauri crate id (not `crate::`)
use rust_todo_app::repo;


// Models live in the root crate (rust-todo-core -> crate id rust_todo_core)
use rust_todo_core::models::{CreateTodo, UpdateTodo, Priority};

// Migrations dir is at repo root: ./migrations (relative to this test file: ../migrations)
#[sqlx::test(migrations = "../migrations")]
async fn create_list_get_update_toggle_delete(pool: PgPool) -> anyhow::Result<()> {
    // create
    let created = repo::create(&pool, CreateTodo {
        title: "Test from repo".into(),
        description: None,
        priority: Priority::Medium,
    }).await?;
    assert!(!created.is_completed, "new todos should start incomplete");

    // list
    let all = repo::list(&pool).await?;
    assert!(all.iter().any(|t| t.id == created.id));

    // get
    let got = repo::get_by_id(&pool, created.id).await?;
    assert_eq!(got.title, "Test from repo");

    // update (partial)
    let updated = repo::update(&pool, created.id, UpdateTodo {
        title: Some("Renamed".into()),
        description: None,
        priority: None,
        is_completed: None,
    }).await?;
    assert_eq!(updated.title, "Renamed");

    // toggle -> true (idempotent)
    let a = repo::toggle(&pool, created.id, true).await?;
    let b = repo::toggle(&pool, created.id, true).await?;
    assert!(b.is_completed);
    assert_eq!(a.completed_at, b.completed_at, "toggle(true) should be idempotent");

    // delete
    let rows = repo::delete(&pool, created.id).await?;
    assert_eq!(rows, 1);
    let rows2 = repo::delete(&pool, created.id).await?;
    assert_eq!(rows2, 0);

    Ok(())
}
