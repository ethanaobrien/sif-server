use base64::{Engine as _, engine::general_purpose};
use openssl::{
    rsa::Rsa,
    sign::Signer,
    pkey::PKey,
    hash::MessageDigest
};
use std::time::{SystemTime, UNIX_EPOCH};
use json::{array, object};
use actix_web::{
    HttpResponse,
    HttpRequest,
    HttpResponseBuilder,
    http::header::HeaderValue
};

pub fn release_info() -> json::JsonValue {
    return array![{"id":423,"key":"UDKkj/dmBRbz+CIB+Ekqyg=="},{"id":1870,"key":"Lckl38UoH8CfOMqMSmMYsA=="},{"id":1871,"key":"acAmAWyPOCrO+R5qY9UTtQ=="},{"id":1872,"key":"LaLzU62pKnTftSEGFhMqfA=="},{"id":1873,"key":"wiaaGZSJexvY0u4poRrGSw=="}]
}

pub fn timestamp() -> u64 {
    let now = SystemTime::now();

    let unix_timestamp = now.duration_since(UNIX_EPOCH).unwrap();
    return unix_timestamp.as_secs();
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
