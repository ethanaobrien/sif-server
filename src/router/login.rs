use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
use crate::router::userdata;
use json::object;
use actix_web::{HttpResponse, HttpRequest};

pub fn authkey(req: HttpRequest) -> HttpResponse {
    //let body = global::read_body(&mut req);
    //println!("{}", body);
    println!("authkey");
    
    let key = "user1";
    
    let uid = userdata::get_uid(key);
    let mut token = String::new();
    general_purpose::STANDARD.encode_string(uid.to_string(), &mut token);
    
    let resp = object!{
        "response_data": {
            "authorize_token": token//,
        //    "dummy_token": body["dummy_token"].clone()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn start_up(req: HttpRequest) -> HttpResponse {
    //let body = global::read_body(&mut req);
    println!("startup");
    
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
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn login(req: HttpRequest) -> HttpResponse {
    //let body = global::read_body(&mut req);
    //println!("{}", body);
    println!("login");
    
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
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn top_info(req: HttpRequest) -> HttpResponse {
    println!("topinfo");
    let resp = object!{
        "response_data":  {
            friend_action_cnt: 0,//1291,
            friend_greet_cnt: 0,
            friend_variety_cnt: 0,//1289,
            friend_new_cnt: 0,
            present_cnt: 0,
            secret_box_badge_flag: false,
            server_datetime: "",
            server_timestamp: global::timestamp(),
            notice_friend_datetime: "",
            notice_mail_datetime: "2019-12-22 13:03:23",
            friends_approval_wait_cnt: 0,
            friends_request_cnt: 0,
            is_today_birthday: false,
            license_info: {
                license_list: [],
                licensed_info: [],
                expired_info: [],
                badge_flag: false
            },
            using_buff_info: [],
            is_klab_id_task_flag: false,
            klab_id_task_can_sync: false,
            has_unread_announce: false, // true,
            live_skip_open_flag: true,
            exchange_badge_cnt: [
                493,
                12,
                345
            ],
            ad_flag: true,
            has_ad_reward: true
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
