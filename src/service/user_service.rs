use crate::errors::*;
use crate::model::*;

pub struct UserService {
    op: UserOp,
}

impl UserService {
    pub fn new(op: UserOp) -> Self {
        UserService { op }
    }

    pub async fn save(&self, user: &User) -> Result<(), Error> {
        self.op.insert(user).await?;

        Ok(())
    }

    pub async fn find_by_id(&self, id: i32) -> Result<Option<User>, Error> {
        let ret = self.op.find_by_id(id).await?;

        Ok(ret)
    }
}
