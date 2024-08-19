mod handlers;
mod models;
mod persistence;

use std::sync::Arc;

use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenvy;
use handlers::*;
pub use models::*;
use persistence::{
    answers_dao::{AnswersDao, AnswersDaoImpl},
    questions_dao::{QuestionsDao, QuestionsDaoImpl},
};
use pretty_env_logger;
use sqlx::postgres;

#[derive(Clone)]
pub struct AppState {
    pub questions_dao: Arc<dyn QuestionsDao + Send + Sync>,
    pub answers_dao: Arc<dyn AnswersDao + Send + Sync>,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    dotenvy::dotenv().expect("Could not load environment variables");

    let url = std::env::var("DATABASE_URL").expect("Unable to get database url");

    let pool = postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("Unable to create database pool");

    let questions_dao = QuestionsDaoImpl::new(pool.clone());
    let answers_dao = AnswersDaoImpl::new(pool.clone());

    let app_state = AppState {
        questions_dao: Arc::new(questions_dao),
        answers_dao: Arc::new(answers_dao),
    };

    let app = Router::new()
        .route("/question", post(create_question))
        .route("/questions", get(read_questions))
        .route("/question", delete(delete_question))
        .route("/answer", post(create_answer))
        .route("/answers", get(read_answers))
        .route("/answer", delete(delete_answer))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .unwrap();

    axum::serve(listener, app).await.unwrap();
}
