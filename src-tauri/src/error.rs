use sqlx::Error as SqlxError;

/// Convert SQLx errors to user-friendly error messages.
/// Usage: `.map_err(to_user_error)`
pub fn to_user_error(err: SqlxError) -> String {
    // Helper: in debug include the underlying error for faster diagnosis;
    // in release keep messages clean for end users.
    #[inline]
    fn dev_or(fallback: &str, err: &SqlxError) -> String {
        if cfg!(debug_assertions) {
            format!("{fallback}: {err}")
        } else {
            fallback.to_string()
        }
    }

    match err {
        // Not found => clear, stable message for UI
        SqlxError::RowNotFound => "Todo not found".to_string(),

        // Database-originated errors (from Postgres)
        SqlxError::Database(db_err) => {
            if let Some(code) = db_err.code() {
                match code.as_ref() {
                    // unique_violation
                    "23505" => "A todo with this information already exists".to_string(),
                    // foreign_key_violation
                    "23503" => "Referenced item does not exist".to_string(),
                    // not_null_violation
                    "23502" => "Required field is missing".to_string(),
                    // anything else -> short message; show details only in debug
                    _ => dev_or("Database error", &SqlxError::Database(db_err)),
                }
            } else {
                dev_or("Database error", &SqlxError::Database(db_err))
            }
        }

        // Pool/runtime conditions that are common in desktop apps
        SqlxError::PoolTimedOut => "Database connection timeout. Please try again.".to_string(),
        SqlxError::PoolClosed => "Database connection is closed.".to_string(),
        // Present in sqlx >= 0.7 — keep it graceful if it occurs
        #[allow(deprecated)]
        SqlxError::WorkerCrashed => dev_or("Database worker crashed", &SqlxError::WorkerCrashed),

        // Everything else
        other => {
            #[cfg(debug_assertions)]
            {
                format!("Database error: {other}")
            }
            #[cfg(not(debug_assertions))]
            {
                "An error occurred while processing your request".to_string()
            }
        }
    }
}
