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

#[tokio::main]
async fn main() {
    dotenv().expect("Failed to load .env file. Create a .env file in the root directory using the .env.example template.");
    let db_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = match postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
    {
        Ok(pool) => {
            println!("Database connection established successfully.");
            pool
        }
        Err(e) => {
            eprintln!("Failed to connect to the database: {}", e);
            std::process::exit(1);
        }
    };

    let app = route::create_router(Arc::new(AppState { db: pool }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server started successfully at 0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}
