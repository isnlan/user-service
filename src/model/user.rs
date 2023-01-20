use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use super::Dao;
use crate::errors::Error;

#[derive(Serialize, Deserialize, Debug, Clone, Default, sqlx::FromRow)]
pub struct User {
    pub id: Option<i32>,
    pub name: String,
    pub age: i32,
    pub pwd: String,
}

#[derive(Clone)]
pub struct UserOp {
    pool: sqlx::PgPool,
}

impl UserOp {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl Dao for UserOp {
    type Model = User;

    async fn insert(&self, model: &Self::Model) -> Result<bool, Error> {
        match sqlx::query("INSERT INTO users (name, age, pwd) VALUES ($1, $2, $3)")
            .bind(&model.name)
            .bind(model.age)
            .bind(&model.pwd)
            .execute(&self.pool)
            .await
        {
            Ok(_) => Ok(true),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        match sqlx::query_as("SELECT id, name, age, pwd from users")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
        {
            Ok(v) => Ok(v),
            Err(error) => {
                tracing::event!(
                    tracing::Level::ERROR,
                    code = error
                        .as_database_error()
                        .unwrap()
                        .code()
                        .unwrap()
                        .parse::<i32>()
                        .unwrap(),
                    db_message = error.as_database_error().unwrap().message(),
                    constraint = error.as_database_error().unwrap().constraint().unwrap()
                );
                Err(Error::DatabaseQueryError(error))
            }
        }
    }
}
