use crate::{recommender::get_top_k, shared::val2json, Context, Response};
use hyper::StatusCode;
use serde::Deserialize;

#[derive(Deserialize)]
struct RecoRequest {
    liked: String,
    k: i32,
    subjects: String,
}

pub async fn test_handler(ctx: Context) -> String {
    format!("hello world! state_thing was: {}", ctx.state.state_thing)
}

pub async fn recom_handler(mut ctx: Context) -> Response {
    // get the request body
    let body: RecoRequest = match ctx.body_json().await {
        Ok(v) => v,
        Err(e) => {
            return hyper::Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(format!("could not parse JSON: {}", e).into())
                .unwrap()
        }
    };

    let liked_course = body.liked;
    let k = body.k;
    let subjects = body.subjects;
    match get_top_k(&liked_course, k, &subjects) {
        Ok(v) => {
            let json_response = val2json(v);
            Response::new(format!("{}", json_response).into())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Response::new(format!("liked course not found: {}", liked_course).into())
        }
    }
}
