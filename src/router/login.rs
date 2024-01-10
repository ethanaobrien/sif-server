use base64::{Engine as _, engine::general_purpose};
use crate::router::{global, userdata};
use json::object;
use actix_web::{HttpResponse, HttpRequest};

pub fn authkey(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    
    //let key = "user1";
    
    //let uid = userdata::get_uid(key);
    let token = general_purpose::STANDARD.encode(String::from("okay"));
    
    let resp = object!{
        "response_data": {
            "authorize_token": token,
            "dummy_token": body["dummy_token"].to_string()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn start_up(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let key = body["devtoken"].to_string().replace(":", "");
    
    let userdata = userdata::get_acc(&key);
    let uid = userdata["user_id"].as_i32().unwrap();
    
    let resp = object!{
        "response_data": {
            "user_id": uid,
            "release_info": global::release_info()
        },
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn login(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let key = body["devtoken"].to_string().replace(":", "");
    
    let userdata = userdata::get_acc(&key);
    let uid = userdata["user_id"].as_i32().unwrap();
    let token = general_purpose::STANDARD.encode(key);
    
    let resp = object!{
        "response_data": {
            "authorize_token": token,
            "user_id": uid,
            "review_version": "",
            "server_timestamp": global::timestamp(),
            "idfa_enabled": false,
            "skip_login_news":true
        },
        "release_info": global::release_info(),
        "status_code": 200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn top_info(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let key = body["devtoken"].to_string().replace(":", "");
    let userdata = userdata::get_acc(&key);
    
    let resp = object!{
        "response_data": userdata["user_data"].clone(),
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
