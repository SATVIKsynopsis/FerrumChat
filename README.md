# FerrumChat 💬

FerrumChat is a robust, real-time backend API for a chat application built with Rust. It leverages the Axum web framework for HTTP routing, WebSockets for real-time bidirectional communication, and PostgreSQL for persistent data storage using `sqlx`.

## ✨ Features

- **User Authentication:** Secure user registration and login with JWT-based session management (via HttpOnly cookies) and Argon2 password hashing.
- **Real-Time Messaging:** Instant message delivery using WebSockets.
- **One-to-One Chats:** Create private chat sessions between two users.
- **Message Management:** Edit and delete specific messages within a chat.
- **User Discovery:** Search for other users by username.
- **Database Integration:** Fully asynchronous PostgreSQL database interactions and connection pooling with `sqlx`.

## 🛠️ Tech Stack

- **Language:** Rust (Edition 2024)
- **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
- **Database:** PostgreSQL
- **Query Builder/ORM:** [SQLx](https://github.com/launchbadge/sqlx)
- **Real-time:** WebSockets (`axum::extract::ws`)
- **Authentication:** JWT (`jsonwebtoken`), Argon2
- **Validation:** `validator`

## 🚀 Getting Started

### Prerequisites

Ensure you have the following installed:
- [Rust](https://www.rust-lang.org/tools/install) (latest stable version)
- PostgreSQL database
- `sqlx-cli` (for running database migrations)
  ```bash
  cargo install sqlx-cli --no-default-features --features rustls,postgres


