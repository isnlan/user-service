mod request;
mod response;
mod user;

use async_trait::async_trait;
pub use request::*;
pub use response::*;
pub use user::*;

use crate::errors::Error;

#[async_trait]
pub trait Dao {
    type Model;

    async fn insert(&self, model: &Self::Model) -> Result<bool, Error>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Self::Model>, Error>;
}
