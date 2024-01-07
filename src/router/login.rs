use server::Request;
use crate::router::global;
use json::object;

//todo **first** - user database
const UID: i32 = 10000001;

pub fn authkey(mut req:Request) {
    let resp = object!{
        "response_data": {
            "authorize_token": "ZjEyOTk4ZjRjNWUyMTE4NTM5ZjI5NTgwNGNlNjhmYzBiNzYxNTA4ZGI1MTJjZWUzNzJiMjExNDNlMDFjNmI2OV9fX181ODk2OTY2Nw==",
            "user_id": UID,
            "review_version": "",
            "server_timestamp": global::timestamp(),
            "idfa_enabled": false,
            "skip_login_news":true
        },
        "release_info": [{"id":423,"key":"UDKkj/dmBRbz+CIB+Ekqyg=="},{"id":1870,"key":"Lckl38UoH8CfOMqMSmMYsA=="},{"id":1871,"key":"acAmAWyPOCrO+R5qY9UTtQ=="},{"id":1872,"key":"LaLzU62pKnTftSEGFhMqfA=="},{"id":1873,"key":"wiaaGZSJexvY0u4poRrGSw=="}],
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
