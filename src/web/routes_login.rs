use axum::{Json, Router, routing::post};
use serde::Deserialize;
use serde_json::{Value, json};
use crate::{Error, Result};

#[derive(Debug, Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}

async fn api_login(payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api-login", "HANDLER");

    // TODO: Implement real db/auth logic
    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies

    // Create the success body
    let body = Json(json!({
        "result": {
            "success": true
        }
    }));

    Ok(body)
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}