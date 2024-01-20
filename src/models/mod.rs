use serde::{Serialize, Deserialize};

pub mod post;
pub mod thread;
pub mod user;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub timestamp: String,
    pub status: u16,
    pub message: String,
    pub path: String,
}