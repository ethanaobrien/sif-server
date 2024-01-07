use server::Request;
use crate::router::global;
use crate::router::userdata;
use json::object;

//todo **first** - user database
const UID: i32 = 10000001;

pub fn authkey(mut req: Request) {
    let body = global::read_body(&mut req);
    println!("{}", json::stringify(body));
    
    let resp = object!{
        "response_data": {
            "authorize_token": "ZjEyOTk4ZjRjNWUyMTE4NTM5ZjI5NTgwNGNlNjhmYzBiNzYxNTA4ZGI1MTJjZWUzNzJiMjExNDNlMDFjNmI2OV9fX181ODk2OTY2Nw==",
            "user_id": UID,
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

pub fn start_up(mut req:Request) {
    
}

pub fn login(mut req:Request) {
    
}

pub fn top_info(mut req:Request) {
    
}
