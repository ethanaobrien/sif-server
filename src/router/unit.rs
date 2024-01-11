use crate::router::global;
use json::{array, object};
use base64::{Engine as _, engine::general_purpose};
use actix_web::{HttpResponse, HttpRequest, http::header::HeaderValue};
use crate::router::userdata;

pub fn deck(req: HttpRequest, body: String) -> HttpResponse {
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    
    let body = global::process_body(body);
	let mut new_deck_info = array![];
    for (_i, deck) in body["unit_deck_list"].members().enumerate() {
        let main = if deck["main_flag"].to_string() == "1" { true } else { false };
        new_deck_info.push(object!{
			unit_deck_id: deck["unit_deck_id"].clone(),
            main_flag: main,
            deck_name: deck["deck_name"].clone(),
			unit_owning_user_ids: deck["unit_deck_detail"].clone()
		}).unwrap();
    }
    let mut data = userdata::get_acc(&key);
    data["deck_info"] = new_deck_info;
    userdata::save_acc(&key, data);
    let resp = object!{
        "response_data": {
            server_timestamp: global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}

pub fn deckname(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let mut data = userdata::get_acc(&key);
	let mut deck_info = data["deck_info"].clone();
	for (i, deck) in deck_info.members().enumerate() {
        if deck["unit_deck_id"].to_string() == body["unit_deck_id"].to_string() && deck[i]["deck_name"] != body["deck_name"] {
			deck_info[i]["deck_name"] = body["deck_name"].clone();
			break
		}
	}
    data["deck_info"] = deck_info;
    userdata::save_acc(&key, data);
    let resp = object!{
        "response_data": {
            server_timestamp: global::timestamp()
        },
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
