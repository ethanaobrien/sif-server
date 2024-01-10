//todo

//use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
//use crate::router::userdata;
use json::object;
use actix_web::{HttpResponse, HttpRequest};

pub fn progress(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "response_data": {},
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
