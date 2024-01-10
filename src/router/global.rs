use base64::{Engine as _, engine::general_purpose};
use openssl::{
    rsa::Rsa,
    sign::Signer,
    pkey::PKey,
    hash::MessageDigest
};
use chrono::{DateTime, Utc};
use std::time::{SystemTime, UNIX_EPOCH};
use json::{array, object};
use actix_web::{
    HttpResponse,
    HttpRequest,
    HttpResponseBuilder,
    http::header::HeaderValue
};

pub fn login_result() -> json::JsonValue {
    return object!{
        friend_action_cnt: 0,//1291,
        friend_greet_cnt: 0,
        friend_variety_cnt: 0,//1289,
        friend_new_cnt: 0,
        present_cnt: 0,
        secret_box_badge_flag: false,
        server_datetime: timestamp_str(),
        server_timestamp: timestamp(),
        notice_friend_datetime: timestamp_str(),
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
    }
}

pub fn release_info() -> json::JsonValue {
    return array![{"id":423,"key":"UDKkj/dmBRbz+CIB+Ekqyg=="},{"id":1870,"key":"Lckl38UoH8CfOMqMSmMYsA=="},{"id":1871,"key":"acAmAWyPOCrO+R5qY9UTtQ=="},{"id":1872,"key":"LaLzU62pKnTftSEGFhMqfA=="},{"id":1873,"key":"wiaaGZSJexvY0u4poRrGSw=="}]
}

pub fn timestamp() -> u64 {
    let now = SystemTime::now();

    let unix_timestamp = now.duration_since(UNIX_EPOCH).unwrap();
    return unix_timestamp.as_secs();
}

pub fn timestamp_str() -> String {
    let now: DateTime<Utc> = Utc::now();
    let formatted_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    return formatted_date;
}

const PRIVKEY: &[u8] = include_bytes!("../../assets/priv.pem");

pub fn sign(req: &HttpRequest, body: &str) -> HttpResponseBuilder {
    let mut rv = HttpResponse::Ok();
    let blank_header = HeaderValue::from_static("");
    let msg_code_val = req.headers().get("x-message-code").unwrap_or(&blank_header).to_str().unwrap_or("");
    if msg_code_val != "" {
        let msg_code = format!("{}{}", body.to_owned(), msg_code_val);

        let rsa = Rsa::private_key_from_pem(PRIVKEY).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let mut signer = Signer::new(MessageDigest::sha1(), &pkey).unwrap();
        signer.update(msg_code.as_bytes()).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        let msg_sign_base64 = general_purpose::STANDARD_NO_PAD.encode(&signature);

        rv.insert_header(("X-Message-Sign", msg_sign_base64.as_str()));
    }
    //req.set_header("content-length", &bytes.len().to_string());
    return rv;
    //return rv;
}

pub fn process_body(body: String) -> json::JsonValue {
    let result = body.split("\r\n\r\n").collect::<Vec<_>>()[1].split("\r\n").collect::<Vec<_>>()[0];
    return json::parse(result).unwrap_or(object!{});
}
