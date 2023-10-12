use axum::{extract, http};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool};

// #[derive(Serialize, FromRow)]
// pub struct Quote {
//     id: uuid::Uuid,
//     book: String,
//     quote: String,
//     inserted_at: chrono::DateTime<chrono::Utc>,
//     updated_at: chrono::DateTime<chrono::Utc>,
// }

// impl Quote {
//     fn new(book: String, quote: String) -> Self {
//         let now = chrono::Utc::now();
//         Self {
//             id: uuid::Uuid::new_v4(),
//             book,
//             quote,
//             inserted_at: now,
//             updated_at: now,
//         }
//     }
// }

#[derive(Debug, Deserialize)]
pub struct CreateQuote {
    book: String,
    quote: String,
}

pub async fn health() -> http::StatusCode {
    http::StatusCode::OK
}