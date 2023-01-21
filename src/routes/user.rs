use std::sync;

use axum::{extract::State, Json};

use crate::{
    errors::Error,
    model::{self, User},
    service,
};

pub async fn get_user_list(
    mgr: State<sync::Arc<service::Manager>>,
) -> Result<model::Response<model::ResponseList<User>>, Error> {
    let mut list = vec![];

    if let Some(user) = mgr.user_serice.find_by_id(1).await? {
        list.push(user);
    }

    let resp = model::ResponseList {
        total: 1,
        list: list,
    };

    Ok(model::Response::ok(Some(resp)))
}

pub async fn create_user(
    mgr: State<sync::Arc<service::Manager>>,
    Json(req): Json<model::RequestCreateUser>,
) -> Result<model::Response<User>, Error> {
    let req = req;

    let user = model::User {
        id: None,
        name: req.name,
        age: req.age,
        pwd: req.pwd,
    };

    mgr.user_serice.save(&user).await?;

    Ok(model::Response::ok(Some(user)))
}
