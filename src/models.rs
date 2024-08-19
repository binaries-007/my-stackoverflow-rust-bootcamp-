use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Deserialize, Serialize)]
pub struct Question {
    pub title: String,
    pub description: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct QuestionDetail {
    pub question_uuid: String,
    pub title: String,
    pub description: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct QuestionId {
    pub question_uuid: String,
}

#[derive(Deserialize, Serialize)]
pub struct Answer {
    pub question_uuid: String,
    pub content: String,
}

#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
pub struct AnswerDetail {
    pub answer_uuid: String,
    pub question_uuid: String,
    pub content: String,
    pub created_at: String,
}

#[derive(Deserialize, Serialize)]
pub struct AnswerId {
    pub answer_uuid: String,
}

#[derive(Error, Debug)]
pub enum DBError {
    #[error("Invalid UUID provided: {0}")]
    InvalidUUID(String),
    #[error("Database error occured")]
    Other(#[from] Box<dyn std::error::Error + Send + Sync>),
}

pub mod postgres_error_codes {
    pub const FOREIGN_KEY_VIOLATION: &str = "23503";
}
