// src/handlers.rs
use axum::{
    extract::{State, Json},
    response::Json as ResponseJson,
    http::StatusCode,
};
use std::sync::Arc;
use chrono::Utc;

use crate::{
    models::*,
    stellar::StellarClient,
    crypto::{generate_key, encrypt_content, decrypt_content},
    database::queries,
    AppState,
};

pub async fn health_check() -> &'static str {
    "OK"
}

pub async fn register_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<RegisterRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    let stellar_client = StellarClient::new();
    
    // Verificar se usuário já existe
    if let Ok(Some(_)) = queries::find_user_by_username(&state.pool, &payload.username).await {
        return Err(StatusCode::CONFLICT);
    }
    
    // Criar conta Stellar
    let keypair = stellar_client
        .create_account()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Criar usuário no banco
    let user = queries::create_user(
        &state.pool,
        &payload.username,
        &keypair.public_key,  // Agora acessando corretamente
        &keypair.secret_key,  // Agora acessando corretamente
        &payload.user_type,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(user.into()))
}

pub async fn login_user(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LoginRequest>,
) -> Result<ResponseJson<UserResponse>, StatusCode> {
    let user = queries::find_user_by_username(&state.pool, &payload.username)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    Ok(ResponseJson(user.into()))
}

pub async fn create_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateProcessRequest>,
) -> Result<ResponseJson<ProcessResponse>, StatusCode> {
    let encryption_key = generate_key();
    let encrypted_content = encrypt_content(&payload.confidential_content, &encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Para MVP, usar um client_id fixo
    let client_id = "temp_client_id";

    let process = queries::create_process(
        &state.pool,
        client_id,
        &payload.title,
        &encrypted_content,
        &encryption_key,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(process.into()))
}

pub async fn share_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<ShareProcessRequest>,
) -> Result<ResponseJson<ProcessShare>, StatusCode> {
    let stellar_client = StellarClient::new();
    
    // Buscar processo
    let _process = queries::find_process_by_id(&state.pool, &payload.process_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Para MVP, usar chaves mock
    let mock_secret = "SDXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";

    // Enviar transação Stellar
    let tx_hash = stellar_client
        .send_access_transaction(
            mock_secret,
            &payload.supplier_public_key,
            &payload.process_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Registrar compartilhamento
    let share = queries::create_process_share(
        &state.pool,
        &payload.process_id,
        &payload.supplier_public_key,
        &tx_hash,
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(share))
}

pub async fn access_process(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<AccessProcessRequest>,
) -> Result<ResponseJson<ProcessAccessResponse>, StatusCode> {
    let stellar_client = StellarClient::new();
    
    // Buscar processo
    let process = queries::find_process_by_id(&state.pool, &payload.process_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    // Para MVP, simular verificação de acesso
    let has_access = stellar_client
        .verify_access_transaction(
            "GDXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
            &payload.supplier_public_key,
            &payload.process_id,
        )
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !has_access {
        return Err(StatusCode::FORBIDDEN);
    }

    // Descriptografar conteúdo
    let decrypted_content = decrypt_content(&process.encrypted_content, &process.encryption_key)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Registrar acesso
    let _access = queries::create_process_access(
        &state.pool,
        &payload.process_id,
        "temp_supplier_id",
    )
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = ProcessAccessResponse {
        process_id: payload.process_id,
        title: process.title,
        content: decrypted_content,
        accessed_at: Utc::now(),
    };

    Ok(ResponseJson(response))
}

pub async fn list_processes(
    State(state): State<Arc<AppState>>,
) -> Result<ResponseJson<Vec<ProcessResponse>>, StatusCode> {
    let processes = queries::list_processes_by_client(&state.pool, "temp_client_id")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response: Vec<ProcessResponse> = processes.into_iter().map(|p| p.into()).collect();
    Ok(ResponseJson(response))
}

pub async fn get_notifications(
    State(state): State<Arc<AppState>>,
) -> Result<ResponseJson<Vec<ProcessAccessWithDetails>>, StatusCode> {
    let notifications = queries::list_process_accesses_by_client(&state.pool, "temp_client_id")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(ResponseJson(notifications))
}