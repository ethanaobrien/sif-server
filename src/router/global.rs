use base64::{Engine as _, engine::general_purpose};
use server::Request;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use std::time::{SystemTime, UNIX_EPOCH};
use json::object;

pub fn release_info() -> json::JsonValue {
    return json::parse(r#"[{"id":423,"key":"UDKkj/dmBRbz+CIB+Ekqyg=="},{"id":1870,"key":"Lckl38UoH8CfOMqMSmMYsA=="},{"id":1871,"key":"acAmAWyPOCrO+R5qY9UTtQ=="},{"id":1872,"key":"LaLzU62pKnTftSEGFhMqfA=="},{"id":1873,"key":"wiaaGZSJexvY0u4poRrGSw=="}]"#).unwrap();
}

pub fn timestamp() -> String {
    let now = SystemTime::now();

    let unix_timestamp = now.duration_since(UNIX_EPOCH).unwrap();
    return unix_timestamp.as_secs().to_string();
}

const PRIVKEY: &[u8] = include_bytes!("../../assets/priv.pem");

pub fn sign_and_send(req: &mut Request, body: &str) {
    if req.get_header("x-message-code") != "" {
        let msg_code = format!("{}{}", body.to_owned(), req.get_header("x-message-code"));

        let rsa = Rsa::private_key_from_pem(PRIVKEY).unwrap();
        let pkey = PKey::from_rsa(rsa).unwrap();
        let mut signer = Signer::new(MessageDigest::sha1(), &pkey).unwrap();
        signer.update(msg_code.as_bytes()).unwrap();
        let signature = signer.sign_to_vec().unwrap();
        let msg_sign_base64 = general_purpose::STANDARD_NO_PAD.encode(&signature);

        req.set_header("X-Message-Sign", &msg_sign_base64);
    }
    let bytes = body.as_bytes();
    req.set_header("content-length", &bytes.len().to_string());
    req.write(bytes);
    req.end();
}

pub fn read_body(req: &mut Request) -> json::JsonValue {
    let body = req.read_all_string();
    let result = body.split("\n").collect::<Vec<_>>()[3].split("\n").collect::<Vec<_>>()[0];
    return json::parse(result).unwrap_or(object!{});
}
