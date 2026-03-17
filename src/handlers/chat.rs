use axum::extract::ws::Message;
use chrono::{DateTime, Utc};
use futures_util::{SinkExt, StreamExt};
use std::sync::Arc;
use tracing;
use uuid::Uuid;

use crate::{AppState, db::MessageExt, dtos::ChatDto, dtos::WsMessageOut, handlers::users_chat};

pub async fn read_loop(
    mut receiver: futures_util::stream::SplitStream<axum::extract::ws::WebSocket>,
    user_id: Uuid,
    state: Arc<AppState>,
) {
    while let Some(Ok(msg)) = receiver.next().await {
        let Message::Text(text) = msg else { continue };

        let payload: ChatDto = match serde_json::from_str(&text) {
            Ok(p) => p,
            Err(_) => continue,
        };

        if payload.r#type.as_deref() == Some("ping") {
            let pong = serde_json::json!({
                "type": "pong",
                "sent": payload.sent.unwrap_or(0)
            });

            let _ =
                users_chat::send_to_user(user_id, Message::Text(pong.to_string().into()), &state)
                    .await;

            continue;
        }

        let other_user = payload.receiver_id;
        let out = WsMessageOut {
            sender_id: user_id,
            content: payload.content.clone(),
            chat_id: payload.chat_id,
            created_at: Utc::now(),
        };

        let message = Message::Text(serde_json::to_string(&out).unwrap().into());

        let _ = users_chat::send_to_user(
            other_user,
            Message::Text(serde_json::to_string(&out).unwrap().into()),
            &state,
        )
        .await;

        let _ = users_chat::send_to_user(user_id, message, &state).await;

        let state_bg = state.clone();
        let content = payload.content.clone();
        let chat_id = payload.chat_id;
        tokio::spawn(async move {
            let _ = state_bg
                .db_client
                .create_message(chat_id, user_id, content.as_str())
                .await;
        });

        tracing::debug!("Sending to user: {}", other_user);

        let sessions = state.active_sessions.lock().await;
        tracing::debug!("Active sessions: {:?}", sessions.keys().collect::<Vec<_>>());
        drop(sessions);
    }
}

pub async fn write_loop(
    mut sender: futures_util::stream::SplitSink<axum::extract::ws::WebSocket, Message>,
    mut rx: tokio::sync::mpsc::UnboundedReceiver<Message>,
    user_id: Uuid,
) {
    while let Some(message) = rx.recv().await {
        if sender.send(message).await.is_err() {
            break;
        }
    }
}
