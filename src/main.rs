// src/main.rs
use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use std::sync::Arc;
use tracing_subscriber;

mod models;
mod handlers;
mod stellar;
mod crypto;
mod database;

use handlers::*;
use database::init_database;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Inicializar logging
    tracing_subscriber::fmt::init();
    
    // Carregar variáveis de ambiente
    dotenv::dotenv().ok();

    // Inicializar banco de dados
    let pool = init_database().await?;

    let app_state = Arc::new(AppState { pool });

    // Configurar rotas
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/api/users/register", post(register_user))
        .route("/api/users/login", post(login_user))
        .route("/api/processes", post(create_process))
        .route("/api/processes", get(list_processes))
        .route("/api/processes/share", post(share_process))
        .route("/api/processes/access", post(access_process))
        .route("/api/notifications", get(get_notifications))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    println!("🚀 Servidor rodando em http://localhost:3000");
    
    axum::serve(listener, app).await?;
    Ok(())
}

pub struct AppState {
    pub pool: sqlx::SqlitePool,
}