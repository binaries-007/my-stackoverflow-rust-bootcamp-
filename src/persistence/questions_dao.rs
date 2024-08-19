use async_trait::async_trait;
use sqlx::PgPool;

use crate::{DBError, Question, QuestionDetail};

#[allow(dead_code)]
#[async_trait]
pub trait QuestionsDao {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError>;
    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError>;
    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError>;
}

#[allow(dead_code)]
pub struct QuestionsDaoImpl {
    db_pool: PgPool,
}

impl QuestionsDaoImpl {
    pub fn new(db: PgPool) -> Self {
        Self { db_pool: db }
    }
}

#[async_trait]
impl QuestionsDao for QuestionsDaoImpl {
    async fn create_question(&self, question: Question) -> Result<QuestionDetail, DBError> {
        let record = sqlx::query!(
            r#"INSERT INTO questions (title, description) VALUES ($1, $2) RETURNING * ;"#,
            question.title,
            question.description
        )
        .fetch_one(&self.db_pool)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(QuestionDetail {
            question_uuid: record.question_uuid.to_string(),
            title: record.title,
            description: record.description,
            created_at: record.created_at.to_string(),
        })
    }

    async fn delete_question(&self, question_uuid: String) -> Result<(), DBError> {
        let question_uuid = sqlx::types::Uuid::parse_str(&question_uuid)
            .map_err(|_| DBError::InvalidUUID(question_uuid))?;

        sqlx::query!(
            r#"DELETE FROM questions WHERE question_uuid = $1"#,
            question_uuid
        )
        .execute(&self.db_pool)
        .await
        .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(())
    }

    async fn get_questions(&self) -> Result<Vec<QuestionDetail>, DBError> {
        let result = sqlx::query!(r#"SELECT * FROM questions ;"#)
            .map(|record| QuestionDetail {
                question_uuid: record.question_uuid.to_string(),
                description: record.description,
                title: record.title,
                created_at: record.created_at.to_string(),
            })
            .fetch_all(&self.db_pool)
            .await
            .map_err(|err| DBError::Other(Box::new(err)))?;

        Ok(result)
    }
}
