use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
use crate::router::userdata;
use json::object;
use actix_web::{HttpResponse, HttpRequest};
use actix_web::http::header::HeaderValue;

pub fn user_info(req: HttpRequest, body: String) -> HttpResponse {
    //let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let userdata = userdata::get_acc(&key);

    let resp = object!{
        "response_data": userdata["user_info"].clone(),
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn lbonus_execute(req: HttpRequest, body: String) -> HttpResponse {
    let resp = include_str!("lbonus.json");
    let resp = resp.replace("timestamppp", &global::timestamp().to_string());
    
    let body = json::stringify(json::parse(&resp).unwrap());
    global::sign(&req, &body).body(body)
}
