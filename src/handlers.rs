use axum::{
    extract::{OriginalUri, Extension},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use validator::Validate;
use clap::{crate_description, crate_name, crate_version};
use serde_json::json;
use serde_json::Value;
use axum_macros::debug_handler;
use axum::extract::RawQuery;
use hyper::Response;
use hyper::Body;

use crate::error::Error as RestError;
use crate::State;
use crate::acls::AclDefinition;

// This is required in order to get the method from the request
#[derive(Debug)]
pub struct RequestMethod(pub hyper::Method);

pub async fn health() -> Json<Value> {
    log::info!("{{\"fn\": \"health\", \"method\":\"get\"}}");
    Json(json!({ "msg": "Healthy"}))
}

pub async fn root() -> Json<Value> {
    log::info!("{{\"fn\": \"root\", \"method\":\"get\"}}");
    Json(
        json!({ "version": crate_version!(), "name": crate_name!(), "description": crate_description!()}),
    )
}

#[debug_handler]
pub async fn delete_acl(Extension(state): Extension<State>, Json(payload): Json<AclDefinition>) -> Result<Response<Body>, RestError> {
    payload.validate()?;
    log::info!(
        "{{\"method\": \"DELETE\", \"path\":\"/acls\", \"payload\":{}}}",
        json!(payload));
    Ok(state.delete(payload).await?)
}

#[debug_handler]
pub async fn post_acl(Extension(state): Extension<State>, Json(payload): Json<AclDefinition>) -> Result<Response<Body>, RestError> {
    payload.validate()?;
    log::info!(
        "{{\"method\": \"POST\", \"path\":\"/acls\", \"payload\":{}}}",
        json!(payload));
    Ok(state.post(payload).await?)
}

#[debug_handler]
pub async fn get_acl(Extension(state): Extension<State>, RawQuery(query): RawQuery) -> Result<Response<Body>, RestError> {
    log::info!(
        "{{\"method\": \"GET\", \"path\":\"/acls\", \"query\": \"{}\"}}",
        query.clone().unwrap_or_else(|| "".to_string()));
    Ok(state.get(query).await?)
}

pub async fn handler_404(OriginalUri(original_uri): OriginalUri) -> impl IntoResponse {
    let parts = original_uri.into_parts();
    let path_and_query = parts.path_and_query.expect("Missing post path and query");
    log::info!(
        "{{\"fn\": \"handler_404\", \"method\":\"get\", \"path\":\"{}\"}}",
        path_and_query
    );
    (
        StatusCode::NOT_FOUND,
        "{\"error_code\": 404, \"message\": \"HTTP 404 Not Found\"}",
    )
}
