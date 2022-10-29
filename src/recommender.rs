extern crate xmlrpc;

use xmlrpc::{Error, Request, Value};

pub fn get_top_k(key: &str, k: i32) -> Result<Value, Error> {
    let request = Request::new("top_k_recommendations").arg(key).arg(k);
    let request_result = request.call_url("http://localhost:8080");
    return request_result;
}
