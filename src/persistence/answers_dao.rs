use async_trait::async_trait;
use sqlx::{query, PgPool};

use crate::{postgres_error_codes, Answer, AnswerDetail, DBError};

#[allow(dead_code)]
#[async_trait]
pub trait AnswersDao {
    async fn create_answer(&self, answer: Answer) -> Result<AnswerDetail, DBError>;
    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError>;
    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError>;
}

#[allow(dead_code)]
pub struct AnswersDaoImpl {
    db_pool: PgPool,
}

impl AnswersDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db_pool: db }
    }
}

#[async_trait]
impl AnswersDao for AnswersDaoImpl {
    async fn create_answer(
        &self,
        Answer {
            question_uuid,
            content,
        }: Answer,
    ) -> Result<AnswerDetail, DBError> {
        let question_uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        let record = query!(
            r#"INSERT INTO answers (question_uuid, content) VALUES ($1, $2) RETURNING *;"#,
            question_uuid,
            content
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|err| match err.as_database_error() {
            Some(db_error)
                if db_error.code().map_or(false, |code| {
                    code == postgres_error_codes::FOREIGN_KEY_VIOLATION
                }) =>
            {
                DBError::InvalidUUID(question_uuid.to_string())
            }

            _ => DBError::Other(Box::new(err)),
        })?;

        Ok(AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_answer(&self, answer_uuid: String) -> Result<(), DBError> {
        let answer_uuid = sqlx::types::Uuid::parse_str(&answer_uuid)
            .map_err(|_| DBError::InvalidUUID(answer_uuid))?;

        sqlx::query!(
            r#"DELETE FROM answers WHERE answer_uuid = $1 ;"#,
            answer_uuid
        )
        .execute(&self.db_pool)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(())
    }

    async fn get_answers(&self, question_uuid: String) -> Result<Vec<AnswerDetail>, DBError> {
        let question_uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        let records = query!(
            r#"SELECT * FROM answers WHERE question_uuid = $1 "#,
            question_uuid
        )
        .map(|record| AnswerDetail {
            answer_uuid: record.answer_uuid.to_string(),
            question_uuid: record.question_uuid.to_string(),
            content: record.content,
            created_at: record.created_at.to_string(),
        })
        .fetch_all(&self.db_pool)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(records)
    }
}
