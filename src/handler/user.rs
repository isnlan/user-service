use std::sync;

use actix_web::{
    web::{self},
    Error, HttpResponse,
};

use crate::model;

use super::Controller;

pub async fn get_user_list() -> Result<HttpResponse, Error> {
    let u1 = model::User {
        name: String::from("renjun"),
        age: 30,
        pwd: String::from("123456"),
    };

    let list = vec![u1];

    let resp = model::ResponseList {
        total: 1,
        list: list,
    };

    model::Response::ok(Some(resp)).to_json_result()
}

pub async fn create_user(
    ctrl: web::Data<sync::Arc<Controller>>,
    req: web::Json<model::RequestCreateUser>,
) -> Result<HttpResponse, Error> {
    let req = req.into_inner();

    ctrl.UserSerice.pr();

    let user = model::User {
        name: req.name,
        age: req.age,
        pwd: req.pwd,
    };

    model::Response::ok(Some(user)).to_json_result()
}
