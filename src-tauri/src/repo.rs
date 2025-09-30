use rust_todo_core::{CreateTodo, UpdateTodo, Todo}; // Priority not used (warning gone)
use sqlx::PgPool;

/// List all todos ordered by creation date (newest first)
pub async fn list(pool: &PgPool) -> Result<Vec<Todo>, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        SELECT
            id,
            title,
            description,
            is_completed,
            priority,
            created_at,
            updated_at,
            completed_at
        FROM todos
        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

/// Get a single todo by ID
pub async fn get_by_id(pool: &PgPool, id: i64) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        SELECT
            id,
            title,
            description,
            is_completed,
            priority,
            created_at,
            updated_at,
            completed_at
        FROM todos
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

/// Create a new todo (timestamps in UTC, completed_at = NULL)
pub async fn create(pool: &PgPool, payload: CreateTodo) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        INSERT INTO todos (title, description, priority, is_completed, created_at, updated_at, completed_at)
        VALUES (
          $1,
          $2,
          $3::priority_level,
          FALSE,
          NOW() AT TIME ZONE 'utc',
          NOW() AT TIME ZONE 'utc',
          NULL
        )
        RETURNING
            id,
            title,
            description,
            is_completed,
            priority,
            created_at,
            updated_at,
            completed_at
        "#
    )
    .bind(payload.title)          // TEXT
    .bind(payload.description)    // Option<TEXT>
    .bind(payload.priority)       // Priority (enum) -> ::priority_level
    .fetch_one(pool)
    .await
}

/// Update an existing todo (partial patch)
/// - COALESCE each field
/// - `updated_at` = NOW() UTC
/// - `completed_at`:
///     * when effective is_completed = true  -> keep existing completed_at or set NOW()
///     * when false                          -> NULL
pub async fn update(pool: &PgPool, id: i64, patch: UpdateTodo) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET
          title        = COALESCE($2, title),
          description  = COALESCE($3, description),
          priority     = COALESCE($4::priority_level, priority),
          is_completed = COALESCE($5, is_completed),
          updated_at   = NOW() AT TIME ZONE 'utc',
          completed_at = CASE
            WHEN COALESCE($5, is_completed) = TRUE
              THEN COALESCE(completed_at, NOW() AT TIME ZONE 'utc')
            ELSE NULL
          END
        WHERE id = $1
        RETURNING
          id,
          title,
          description,
          is_completed,
          priority,
          created_at,
          updated_at,
          completed_at
        "#
    )
    .bind(id)                          // $1
    .bind(patch.title)                 // $2 Option<TEXT>
    .bind(patch.description)           // $3 Option<TEXT>
    .bind(patch.priority)              // $4 Option<Priority> -> ::priority_level
    .bind(patch.is_completed)          // $5 Option<bool>
    .fetch_one(pool)
    .await
}

/// Explicit toggle to the desired completion state
/// - Idempotent: if setting to true, preserves prior `completed_at` (doesn't overwrite)
pub async fn toggle(pool: &PgPool, id: i64, to_completed: bool) -> Result<Todo, sqlx::Error> {
    sqlx::query_as::<_, Todo>(
        r#"
        UPDATE todos
        SET
          is_completed = $2,
          updated_at   = NOW() AT TIME ZONE 'utc',
          completed_at = CASE
            WHEN $2 = TRUE THEN COALESCE(completed_at, NOW() AT TIME ZONE 'utc')
            ELSE NULL
          END
        WHERE id = $1
        RETURNING
          id,
          title,
          description,
          is_completed,
          priority,
          created_at,
          updated_at,
          completed_at
        "#
    )
    .bind(id)
    .bind(to_completed)
    .fetch_one(pool)
    .await
}

/// Delete a todo by ID
/// Returns 1 if deleted, 0 if not found
pub async fn delete(pool: &PgPool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"DELETE FROM todos WHERE id = $1"#
    )
    .bind(id)
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}
