use axum::{
    extract::State, http::StatusCode, routing::{get, post}, Json, Router
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use sqlx::{postgres::PgPoolOptions, PgPool, Pool, Postgres};
use std::time::Duration;

async fn get_db_pool() -> PgPool {
    let db_connection_str = std::env::var("DATABASE_URL")
    .unwrap_or_else(|_| {println!("error!"); "".to_string()});//"postgres://postgres:password@localhost".to_string());
    tracing::info!(db_connection_str);
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&db_connection_str)
        .await
        .expect("can't connect to database")  
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let db_pool = get_db_pool().await;

    let app = Router::new()
        .route("/", get(index))
        .route("/devices", get(list_devices))
        .route("/devices", post(create_device))
        .with_state(db_pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> &'static str {
    "Hello, World!"
}

async fn list_devices()  -> (StatusCode, Json<Vec<Device>>) {
todo!()
}

async fn create_device(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateDevice>,
) -> Result<String, (StatusCode, String)> {
    // ) -> Result<Uuid, (StatusCode, String)> {
    let id = Uuid::now_v7();

    let result = 
    sqlx::query("INSERT INTO devices (id, name) VALUES ($1, $2);")
        .bind(id.to_string())
        .bind(payload.name)
        .execute(&pool)
        .await
        .map_err(|err| {
            println!("{}", err.to_string());
            (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
        });

    match result {
        Ok(_) => Ok( id.to_string() ),
        Err(e) => Err(e),
    }
}

#[derive(Deserialize)]
struct CreateDevice {
    name: String,
}

#[derive(Serialize)]
struct Device {
    id: Uuid,
    name: String,
}
