use chrono::NaiveDateTime;
use serde::Serialize;
use sqlx::{Error, SqlitePool, query_as};

#[derive(Serialize, Clone, sqlx::FromRow)]
pub struct Todo {
    id: i64,
    body: String,
    completed: bool,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}

impl Todo {
    pub async fn list(dbpool: SqlitePool) -> Result<Vec<Todo>, Error> {
        query_as("select id,body,completed,created_at,updated_at from todos")
            .fetch_all(&dbpool)
            .await
    }

    pub async fn read(dbpool: SqlitePool, id: i64) -> Result<Todo, Error> {
        query_as("select id,body,completed,created_at,updated_at from todos where id = ?")
            .bind(id)
            .fetch_one(&dbpool)
            .await
    }
}
