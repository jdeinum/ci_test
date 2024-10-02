use axum::{
    routing::{get, post},
    Router,
};
mod handlers;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // set up DB
    let conn_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be defined");
    let pool = sqlx::postgres::PgPool::connect(&conn_url).await.unwrap();

    // run migrations
    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    // build our application
    let app = Router::new()
        .route("/", get(handlers::root))
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::read_users))
        .with_state(pool);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {}
