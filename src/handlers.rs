use std::{error, sync::Arc};

use axum::{Json, extract::{State, Path}, http::StatusCode, response::IntoResponse, response::Redirect};
use serde_json::json;

use nanoid::nanoid;

use crate::{AppState, model::UrlModel, schema::UrlSchema};

pub async fn hellowworld(State(data): State<Arc<AppState>>) -> impl axum::response::IntoResponse {
    println!("Received request at / endpoint.");
    let json_response = json!({
        "status": "ok",
        "message": "Hello, World!"
    });
    axum::Json(json_response)
}

pub async fn hellotest(
    State(data): State<Arc<AppState>>,
    Path(test): Path<String>
) -> impl axum::response::IntoResponse {
    println!("Received request at /{{test}} (hellotest) endpoint.");
    let json_response = json!({
        "status": "ok",
        "message": test
    });
    axum::Json(json_response)
}

pub async fn create_url(
    State(data): State<Arc<AppState>>,
    Json(body): Json<UrlSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Received request at /create_short_url endpoint.");
    let mut token_lenght = 6;
    loop {

        if token_lenght > 10 {
            let error_response = json!({
                "status": "error",
                "message": "Failed to generate a unique token after multiple attempts"
            });
            return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
        }

        let new_token = nanoid!(token_lenght);
        let new_url = sqlx::query_as!(
            UrlModel,
            r#"
            INSERT INTO urls (token, original_url)
            VALUES ($1, $2)
            RETURNING *
            "#,
            &new_token,
            &body.original_url,
        )
        .fetch_one(&data.db)
        .await
        .map_err(|e| e.to_string());

        match new_url {
            Ok(url) => {
                let success_response = json!({
                    "status": "success",
                    "message": "Short URL created successfully",
                    "token": url.token
                });
                return Ok((StatusCode::CREATED, Json(success_response)));
            }
            Err(err) => {
                if err.contains("duplicate key value") {
                    token_lenght += 1;
                    continue;
                }

                let error_response = json!({
                    "status": "error",
                    "message": err
                });
                return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
            }
        }
    };
}

pub async fn get_original_url(
    State(data): State<Arc<AppState>>,
    Path(token): Path<String>
) -> Result<Redirect, (StatusCode, Json<serde_json::Value>)> {
    println!("Received request at /get_original_url endpoint.");

    let url = sqlx::query_as!(
        UrlModel,
        r#"
        SELECT * FROM urls WHERE token = $1
        "#,
        &token,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    match url {
        Ok(url) => {
            Ok(Redirect::temporary(&url.original_url))
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": err
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn delete_url(
    State(data): State<Arc<AppState>>,
    Path(token): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Received request at /delete_short_url endpoint.");
    let result = sqlx::query!(
        r#"
        DELETE FROM urls WHERE token = $1
        "#,
        &token,
    )
    .execute(&data.db)
    .await
    .map_err(|e| e.to_string());

    match result {
        Ok(_) => {

            let success_response = json!({
                "status": "success",
                "message": "Short URL deleted successfully",
            });
            Ok((StatusCode::OK, Json(success_response)))
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": err
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn get_info_url(
    State(data): State<Arc<AppState>>,
    Path(token): Path<String>
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    println!("Received request at /get_info_url endpoint.");

    let url = sqlx::query_as!(
        UrlModel,
        r#"
        SELECT * FROM urls WHERE token = $1
        "#,
        &token,
    )
    .fetch_one(&data.db)
    .await
    .map_err(|e| e.to_string());

    match url {
        Ok(url) => {
            let success_response = json!({
                "status": "success",
                "message": "Short URL info retrieved successfully",
                "token": url.token,
                "original_url": url.original_url,
                "created_at": url.created_at
            });
            Ok((StatusCode::OK, Json(success_response)))
        }
        Err(err) => {
            let error_response = json!({
                "status": "error",
                "message": err
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

