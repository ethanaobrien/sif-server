use server::Request;
use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
use crate::router::userdata;
use json::object;

pub fn authkey(mut req: Request) {
    let body = global::read_body(&mut req);
    println!("{}", body);
    
    let key = "user1";
    
    let uid = userdata::get_uid(key);
    let mut token = String::new();
    general_purpose::STANDARD.encode_string(uid.to_string(), &mut token);
    
    let resp = object!{
        "response_data": {
            "authorize_token": token,
            "dummy_token": body["dummy_token"].clone()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    global::sign_and_send(&mut req, &json::stringify(resp));
}

pub fn start_up(mut req:Request) {
    let body = global::read_body(&mut req);
    
    let key = "user1";
    
    let uid = userdata::get_uid(key);
    let mut token = String::new();
    general_purpose::STANDARD.encode_string(uid.to_string(), &mut token);
    
    let resp = object!{
        "response_data": {
            "user_id": uid,
            "release_info": global::release_info()
        },
        "status_code":200
    };
    global::sign_and_send(&mut req, &json::stringify(resp));
}

pub fn login(mut req:Request) {
    let body = global::read_body(&mut req);
    println!("{}", body);
    
    let key = "user1";
    
    let uid = userdata::get_uid(key);
    let mut token = String::new();
    general_purpose::STANDARD.encode_string(uid.to_string(), &mut token);
    
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
        "status_code":200
    };
    global::sign_and_send(&mut req, &json::stringify(resp));
}

pub fn top_info(mut req:Request) {
    
}
