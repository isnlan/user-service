use std::sync;

use actix_web::{
    web::{self},
    Error, HttpResponse,
};

use crate::{model, service};

pub async fn get_user_list(
    mgr: web::Data<sync::Arc<service::Manager>>,
) -> Result<HttpResponse, Error> {
    let mut list = vec![];

    if let Some(user) = mgr.user_serice.find_by_id(1).await? {
        list.push(user);
    }

    let resp = model::ResponseList {
        total: 1,
        list: list,
    };

    model::Response::ok(Some(resp)).to_json_result()
}

pub async fn create_user(
    mgr: web::Data<sync::Arc<service::Manager>>,
    req: web::Json<model::RequestCreateUser>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();

    let user = model::User {
        id: None,
        name: req.name,
        age: req.age,
        pwd: req.pwd,
    };

    mgr.user_serice.save(&user).await?;

    model::Response::ok(Some(user)).to_json_result()
}
