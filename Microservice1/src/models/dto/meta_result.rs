use rocket_okapi::JsonSchema;
use serde::{Deserialize, Serialize};
use rocket::http::Header;

#[derive(Responder, Debug, Deserialize, Serialize, JsonSchema)]
pub struct MetaResult {
    pub message: String,
  //  pub response: String,
  //  pub response_code: i32,

}

// impl<T> MetaResult<T> {
//     fn new(res: T, msg: String, code: i32) -> MetaResult<T> {
//         MetaResult {
//             result: res,
//             response_code: code,
//             message: msg,
//         }
//     }
// }