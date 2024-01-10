use crate::router::global;
use json::object;
use actix_web::{HttpResponse, HttpRequest};

pub fn get(req: HttpRequest, _body: String) -> HttpResponse {
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
