use axum::{extract::State, http::StatusCode, Json};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct User {
    id: Uuid,
    name: String,
    added_at: chrono::DateTime<chrono::Utc>,
}

impl User {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            added_at: chrono::Utc::now(),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateUser {
    name: String,
}

// simple health check
pub async fn root() -> StatusCode {
    StatusCode::OK
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUser>,
) -> Result<(StatusCode, axum::Json<User>), StatusCode> {
    let user = User::new(payload.name);

    let res = sqlx::query(
        r#"
        INSERT INTO users (id, name, added_at) 
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(&user.id)
    .bind(&user.name)
    .bind(&user.added_at)
    .execute(&pool)
    .await;

    return match res {
        Ok(_) => Ok((StatusCode::CREATED, axum::Json(user))),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
}

pub async fn read_users(State(pool): State<PgPool>) -> Result<axum::Json<Vec<User>>, StatusCode> {
    let res = sqlx::query_as::<_, User>("SELECT * FROM users")
        .fetch_all(&pool)
        .await;

    match res {
        Ok(quotes) => Ok(axum::Json(quotes)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
