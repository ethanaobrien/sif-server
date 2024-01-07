use base64::{Engine as _, engine::general_purpose};
use server::Request;
use openssl::rsa::Rsa;
use openssl::sign::{Signer, Verifier};
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;

use std::time::{SystemTime, UNIX_EPOCH};

pub fn timestamp() -> String {
    let now = SystemTime::now();

    let unix_timestamp = now.duration_since(UNIX_EPOCH).unwrap();
    return unix_timestamp.as_secs().to_string();
}

const PRIVKEY: &[u8] = include_bytes!("../../assets/priv.pem");

pub fn sign_and_send(req: &mut Request, body: &str) {
    //if req.get_header("x-message-code") == "" {
    //    return;
    //}
    let msg_code = format!("{}{}", body.to_owned(), req.get_header("x-message-code"));

    let rsa = Rsa::private_key_from_pem(PRIVKEY).unwrap();
    let pkey = PKey::from_rsa(rsa).unwrap();
    let mut signer = Signer::new(MessageDigest::sha1(), &pkey).unwrap();
    signer.update(msg_code.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    let msg_sign_base64 = general_purpose::STANDARD_NO_PAD.encode(&signature);

    req.set_header("X-Message-Sign", &msg_sign_base64);
    req.write(body.as_bytes());
    req.end();
}
