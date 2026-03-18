use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, patch, post},
};
use std::sync::Arc;
use uuid::Uuid;

use crate::{
    AppState,
    db::{ChatExt, MessageExt},
    dtos::{
        ChatDto, ChatListResponseDto, CreateChatDto, EditMessageDto, MessageDto,
        MessageListResponseDto, RequestQueryDto,
    },
    error::HttpError,
    middleware::JWTAuthMiddleware,
    models::{Chat, Message},
};
use axum::Extension;

pub fn chats_handler() -> Router<Arc<AppState>> {
    Router::new()
        .route("/chats", post(create_chat))
        .route("/chats", get(get_chats))
        .route("/chats/:id/messages", get(get_messages))
        .route("/chats/:id", delete(delete_chat))
        .route("/messages/:id", patch(edit_message))
        .route("/messages/:id", delete(delete_message))
}

pub async fn create_chat(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Json(body): Json<CreateChatDto>,
) -> Result<impl IntoResponse, HttpError> {
    let result = state
        .db_client
        .create_chat(user.user.id, body.receiver_id)
        .await;

    match result {
        Ok(chat) => Ok((StatusCode::CREATED, Json(chat))),

        Err(e) => {
            if let Some(db_err) = e.as_database_error() {
                if let Some(constraint) = db_err.constraint() {
                    if constraint == "unique_chat_pair" {
                        let existing_chat = state
                            .db_client
                            .get_chat_between_users(user.user.id, body.receiver_id)
                            .await
                            .map_err(|e| HttpError::server_error(e.to_string()))?;

                        return Ok((StatusCode::OK, Json(existing_chat)));
                    }
                }
            }

            Err(HttpError::server_error(e.to_string()))
        }
    }
}

pub async fn get_chats(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
) -> Result<impl IntoResponse, HttpError> {
    let chats = state
        .db_client
        .get_user_chats_with_participants(user.user.id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(chats))
}

pub async fn get_messages(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Path(chat_id): Path<Uuid>,
    Query(query): Query<RequestQueryDto>,
) -> Result<impl IntoResponse, HttpError> {
    let limit = query.limit.unwrap_or(50) as i64;
    let page = query.page.unwrap_or(1) as i64;
    let offset = (page - 1) * limit;

    let messages = state
        .db_client
        .get_chat_messages(chat_id, limit, offset)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(Json(messages))
}

pub async fn delete_chat(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Path(chat_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    state
        .db_client
        .delete_chat(chat_id)
        .await
        .map_err(|e| HttpError::server_error(e.to_string()))?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn edit_message(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Path(message_id): Path<Uuid>,
    Json(body): Json<EditMessageDto>,
) -> Result<impl IntoResponse, HttpError> {
    let message = state
        .db_client
        .edit_message(message_id, user.user.id, &body.content)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => HttpError::unauthorized("Not allowed to edit this message"),
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(Json(message))
}

pub async fn delete_message(
    State(state): State<Arc<AppState>>,
    Extension(user): Extension<JWTAuthMiddleware>,
    Path(message_id): Path<Uuid>,
) -> Result<impl IntoResponse, HttpError> {
    state
        .db_client
        .delete_message(message_id, user.user.id)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                HttpError::unauthorized("Not allowed to delete this message")
            }
            _ => HttpError::server_error(e.to_string()),
        })?;

    Ok(StatusCode::NO_CONTENT)
}
