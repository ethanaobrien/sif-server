use base64::{Engine as _, engine::general_purpose};
use crate::router::global;
use crate::router::userdata;
use json::{array, object};
use actix_web::{HttpResponse, HttpRequest};
use actix_web::http::header::HeaderValue;
use lazy_static::lazy_static;
use json::JsonValue;

lazy_static! {
    static ref STATIC_RESPONSES: JsonValue = array![
        "album.albumAll.result.json",
        "banner.bannerList.result.json",
        "disabled.award.awardInfo.result.json",
        "disabled.background.backgroundInfo.result.json",
        "disabled.live.liveStatus.result.json",
        "disabled.multiunit.multiunitscenarioStatus.result.json",
        "disabled.scenario.scenarioStatus.result.json",
        "disabled.subscenario.subscenarioStatus.result.json",
        "disabled.unit.deckInfo.result.json",
        "disabled.unit.unitAll.result.json",
        "eventscenario.status.result.json",
        "item.list.result.json",
        "live.schedule.result.json",
        "museum.info.result.json",
        "stamp.stampInfo.result.json",
        "unit.removableSkillInfo.result.json",
        "unit.supporterAll.result.json"
    ];
    static ref STATIC_RESPONSE_DATA: JsonValue = array![
        include_str!("main-api/album.albumAll.result.json"),
        include_str!("main-api/banner.bannerList.result.json"),
        include_str!("main-api/disabled.award.awardInfo.result.json"),
        include_str!("main-api/disabled.background.backgroundInfo.result.json"),
        include_str!("main-api/disabled.live.liveStatus.result.json"),
        include_str!("main-api/disabled.multiunit.multiunitscenarioStatus.result.json"),
        include_str!("main-api/disabled.scenario.scenarioStatus.result.json"),
        include_str!("main-api/disabled.subscenario.subscenarioStatus.result.json"),
        include_str!("main-api/disabled.unit.deckInfo.result.json"),
        include_str!("main-api/disabled.unit.unitAll.result.json"),
        include_str!("main-api/eventscenario.status.result.json"),
        include_str!("main-api/item.list.result.json"),
        include_str!("main-api/live.schedule.result.json"),
        include_str!("main-api/museum.info.result.json"),
        include_str!("main-api/stamp.stampInfo.result.json"),
        include_str!("main-api/unit.removableSkillInfo.result.json"),
        include_str!("main-api/unit.supporterAll.result.json")
    ];
}

fn live() -> JsonValue {
    object!{}
}
fn unit() -> JsonValue {
    object!{}
}
fn costume() -> JsonValue {
    object!{}
}
fn album() -> JsonValue {
    object!{}
}
fn scenario() -> JsonValue {
    object!{}
}
fn subscenario() -> JsonValue {
    object!{}
}
fn eventscenario() -> JsonValue {
    object!{}
}
fn multiunit() -> JsonValue {
    object!{}
}
fn payment() -> JsonValue {
    object!{}
}
fn banner() -> JsonValue {
    object!{}
}
fn notice() -> JsonValue {
    object!{}
}
fn user() -> JsonValue {
    object!{}
}
fn navigation() -> JsonValue {
    object!{}
}
fn award() -> JsonValue {
    object!{}
}
fn background() -> JsonValue {
    object!{}
}
fn stamp() -> JsonValue {
    object!{}
}
fn exchange() -> JsonValue {
    object!{}
}
fn livese() -> JsonValue {
    object!{}
}
fn liveicon() -> JsonValue {
    object!{}
}
fn item() -> JsonValue {
    object!{}
}
fn marathon() -> JsonValue {
    object!{}
}
fn challenge() -> JsonValue {
    object!{}
}
fn login() -> JsonValue {
    object!{}
}

pub fn api(req: HttpRequest, body: String) -> HttpResponse {
    let body = global::process_body(body);
    let blank_header = HeaderValue::from_static("");
    let decoded = general_purpose::STANDARD.decode(req.headers().get("authorize").unwrap_or(&blank_header).to_str().unwrap_or("").split("token=").collect::<Vec<_>>().pop().unwrap().split("&").collect::<Vec<_>>()[0]).unwrap();
    let key = String::from_utf8_lossy(&decoded);
    let userdata = userdata::get_acc(&key);
    let mut resp_data = array![];
    
    for (index, data) in body.members().enumerate() {
        println!("main api {} {}", index, data);
        
        let filename = format!("{}.{}.result.json", data["module"], data["action"]);
        if STATIC_RESPONSES.contains(&*filename) {
            println!("Serving static response for {}.{}", data["module"], data["action"]);
            let mut index = 0;
            for (i, data) in STATIC_RESPONSES.members().enumerate() {
                if data.to_string() == filename {
                    index = i;
                    break;
                }
            }
            resp_data.push(json::parse(&STATIC_RESPONSE_DATA[index].to_string()).unwrap()).unwrap();
        }
        
        match data["module"].as_str().unwrap() {
            "live" => { resp_data.push(live()).unwrap(); },
            "unit" => { resp_data.push(unit()).unwrap(); },
            "costume" => { resp_data.push(costume()).unwrap(); },
            "album" => { resp_data.push(album()).unwrap(); },
            "scenario" => { resp_data.push(scenario()).unwrap(); },
            "subscenario" => { resp_data.push(subscenario()).unwrap(); },
            "eventscenario" => { resp_data.push(eventscenario()).unwrap(); },
            "multiunit" => { resp_data.push(multiunit()).unwrap(); },
            "payment" => { resp_data.push(payment()).unwrap(); },
            "banner" => { resp_data.push(banner()).unwrap(); },
            "notice" => { resp_data.push(notice()).unwrap(); },
            "user" => { resp_data.push(user()).unwrap(); },
            "navigation" => { resp_data.push(navigation()).unwrap(); },
            "award" => { resp_data.push(award()).unwrap(); },
            "background" => { resp_data.push(background()).unwrap(); },
            "stamp" => { resp_data.push(stamp()).unwrap(); },
            "exchange" => { resp_data.push(exchange()).unwrap(); },
            "livese" => { resp_data.push(livese()).unwrap(); },
            "liveicon" => { resp_data.push(liveicon()).unwrap(); },
            "item" => { resp_data.push(item()).unwrap(); },
            "marathon" => { resp_data.push(marathon()).unwrap(); },
            "challenge" => { resp_data.push(challenge()).unwrap(); },
            "login" => { resp_data.push(login()).unwrap(); },
            _ => {
                println!("Unhandled main api {}", data["module"]);
            }
        }
    }
    
    let resp = object!{
        "response_data": resp_data,
        "release_info": global::release_info(),
        "status_code":200
    };
    let body = json::stringify(resp);
    global::sign(&req, &body).body(body)
}
