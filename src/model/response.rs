use serde::Serialize;

const SUCCESS_CODE: i32 = 0;
const SUCCESS_MSG: &str = "ok";

#[derive(Serialize, Debug)]
pub struct Response<T>
where
    T: Serialize,
{
    code: i32,
    message: String,
    data: Option<T>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: Option<T>) -> Self {
        Response {
            code: SUCCESS_CODE,
            message: SUCCESS_MSG.to_string(),
            data,
        }
    }

    pub fn to_json_result(&self) -> Result<actix_web::HttpResponse, actix_web::Error> {
        Ok(actix_web::HttpResponse::Ok().json(self))
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseList<T>
where
    T: Serialize,
{
    pub total: i64,
    pub list: Vec<T>,
}
