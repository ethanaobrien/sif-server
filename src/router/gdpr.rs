use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
use crate::router::userdata;
use json::object;
use actix_web::{HttpResponse, HttpRequest};
use actix_web::http::header::HeaderValue;

pub fn get(req: HttpRequest, body: String) -> HttpResponse {
    let resp = object!{
        "response_data": {
            enable_gdpr: true,
            is_eea: false,
            server_timestamp: global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
