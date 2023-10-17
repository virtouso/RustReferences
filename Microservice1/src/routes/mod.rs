use rocket::serde::json::Json;


use rocket::http::Header;

// pub mod auth;
// pub mod leaderboard;


use crate::models::dto::meta_result::MetaResult;


#[rocket_okapi::openapi(tag = "test api")]
#[get("/")]
pub fn index() -> Json<MetaResult> {
    Json(MetaResult {
        message: " hello test player api!".to_string(),
        // response_code: 200,
        //  result: "changiz".to_string(),
        //  header: Header::new("Content-Type".to_string(), "application/json".to_string())
    })
}
