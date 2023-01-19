use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct RequestCreateUser {
    pub name: String,
    pub age: i32,
    pub pwd: String,
}
