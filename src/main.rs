use dotenv::dotenv;
use std::sync::Arc;
use sqlx::{PgPool, postgres};

mod handlers;
mod route;
mod model;
mod schema;

#[derive(Clone)]
pub struct AppState {
    db: PgPool,
}

async fn create_pool(db_url: &str) -> Result<PgPool, sqlx::Error> {
    let pool = postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::migrate!().run(&pool).await?;

    Ok(pool)
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match create_pool(&db_url).await {
        Ok(pool) => pool,
        Err(err) => {
            eprintln!("Failed to create database pool: {}", err);
            std::process::exit(1);
        }
    };

    let app = route::create_router(Arc::new(AppState { db: pool }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
