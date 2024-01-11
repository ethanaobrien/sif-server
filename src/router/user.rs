use base64::{Engine as _, engine::general_purpose};
use crate::router::{global, userdata};
use json::object;
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};

pub fn user_info(req: HttpRequest, _body: String) -> HttpResponse {
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

pub fn set_notification_token(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "response_data": [],
        "release_info": global::release_info(),
        "status_code":200
    };
    
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn lbonus_execute(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = include_str!("lbonus.json");
    let resp = resp.replace("timestamppp", &global::timestamp().to_string());
    
    let body = json::stringify(json::parse(&resp).unwrap());
    global::sign(&req, &body).body(body)
}

pub fn tos(req: HttpRequest, _body: String) -> HttpResponse {
    let resp = object!{
        "response_data": {
            "tos_id": 8,
            "tos_type": 3,
            "is_agreed": true,
            "server_timestamp": global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn change_name(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let mut userdata = userdata::get_acc(&key);
    
    let resp = object!{
        "response_data": {
            before_name: userdata["user_info"]["user"]["name"].clone(),
            after_name: body["name"].clone(),
            "server_timestamp": global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    userdata["user_info"]["user"]["name"] = body["name"].clone();
    userdata::save_acc(&key, userdata);
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
    
}
