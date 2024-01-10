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

fn live() -> Option<JsonValue> {
    None
}
fn unit() -> Option<JsonValue> {
    None
}
fn costume() -> Option<JsonValue> {
    None
}
fn album() -> Option<JsonValue> {
    None
}
fn scenario() -> Option<JsonValue> {
    None
}
fn subscenario() -> Option<JsonValue> {
    None
}
fn eventscenario() -> Option<JsonValue> {
    None
}
fn multiunit() -> Option<JsonValue> {
    None
}
fn payment() -> Option<JsonValue> {
    None
}
fn banner() -> Option<JsonValue> {
    None
}
fn notice() -> Option<JsonValue> {
    None
}
fn user() -> Option<JsonValue> {
    None
}
fn navigation() -> Option<JsonValue> {
    None
}
fn award() -> Option<JsonValue> {
    None
}
fn background() -> Option<JsonValue> {
    None
}
fn stamp() -> Option<JsonValue> {
    None
}
fn exchange() -> Option<JsonValue> {
    None
}
fn livese() -> Option<JsonValue> {
    None
}
fn liveicon() -> Option<JsonValue> {
    None
}
fn item() -> Option<JsonValue> {
    None
}
fn marathon() -> Option<JsonValue> {
    None
}
fn challenge() -> Option<JsonValue> {
    None
}
fn login() -> Option<JsonValue> {
    None
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
        if let Some(d) = live() { resp_data.push(d).unwrap(); }
        
        //messy
        match data["module"].as_str().unwrap() {
            "live" => { if let Some(d) = live() { resp_data.push(d).unwrap(); } },
            "unit" => { if let Some(d) = unit() { resp_data.push(d).unwrap(); } },
            "costume" => { if let Some(d) = costume() { resp_data.push(d).unwrap(); } },
            "album" => { if let Some(d) = album() { resp_data.push(d).unwrap(); } },
            "scenario" => { if let Some(d) = scenario() { resp_data.push(d).unwrap(); } },
            "subscenario" => { if let Some(d) = subscenario() { resp_data.push(d).unwrap(); } },
            "eventscenario" => { if let Some(d) = eventscenario() { resp_data.push(d).unwrap(); } },
            "multiunit" => { if let Some(d) = multiunit() { resp_data.push(d).unwrap(); } },
            "payment" => { if let Some(d) = payment() { resp_data.push(d).unwrap(); } },
            "banner" => { if let Some(d) = banner() { resp_data.push(d).unwrap(); } },
            "notice" => { if let Some(d) = notice() { resp_data.push(d).unwrap(); } },
            "user" => { if let Some(d) = user() { resp_data.push(d).unwrap(); } },
            "navigation" => { if let Some(d) = navigation() { resp_data.push(d).unwrap(); } },
            "award" => { if let Some(d) = award() { resp_data.push(d).unwrap(); } },
            "background" => { if let Some(d) = background() { resp_data.push(d).unwrap(); } },
            "stamp" => { if let Some(d) = stamp() { resp_data.push(d).unwrap(); } },
            "exchange" => { if let Some(d) = exchange() { resp_data.push(d).unwrap(); } },
            "livese" => { if let Some(d) = livese() { resp_data.push(d).unwrap(); } },
            "liveicon" => { if let Some(d) = liveicon() { resp_data.push(d).unwrap(); } },
            "item" => { if let Some(d) = item() { resp_data.push(d).unwrap(); } },
            "marathon" => { if let Some(d) = marathon() { resp_data.push(d).unwrap(); } },
            "challenge" => { if let Some(d) = challenge() { resp_data.push(d).unwrap(); } },
            "login" => { if let Some(d) = login() { resp_data.push(d).unwrap(); } },
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
