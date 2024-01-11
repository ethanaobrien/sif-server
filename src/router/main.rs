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
        "disabled.background.backgroundInfo.result.json",
        "disabled.multiunit.multiunitscenarioStatus.result.json",
        "disabled.scenario.scenarioStatus.result.json",
        "disabled.subscenario.subscenarioStatus.result.json",
        "eventscenario.status.result.json",
        "item.list.result.json",
        "live.schedule.result.json",
        "museum.info.result.json",
        "stamp.stampInfo.result.json",
        "unit.removableSkillInfo.result.json"
    ];
    static ref STATIC_RESPONSE_DATA: JsonValue = array![
        include_str!("main-api/album.albumAll.result.json"),
        include_str!("main-api/banner.bannerList.result.json"),
        include_str!("main-api/disabled.background.backgroundInfo.result.json"),
        include_str!("main-api/disabled.multiunit.multiunitscenarioStatus.result.json"),
        include_str!("main-api/disabled.scenario.scenarioStatus.result.json"),
        include_str!("main-api/disabled.subscenario.subscenarioStatus.result.json"),
        include_str!("main-api/eventscenario.status.result.json"),
        include_str!("main-api/item.list.result.json"),
        include_str!("main-api/live.schedule.result.json"),
        include_str!("main-api/museum.info.result.json"),
        include_str!("main-api/stamp.stampInfo.result.json"),
        include_str!("main-api/unit.removableSkillInfo.result.json")
    ];
}

fn live(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "liveStatus" => {
            Some(object!{
                result: userdata["live_status"].clone(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn unit(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "unitAll" => {
            Some(object!{
                result: userdata::get_unitall(userdata["unit_all"].clone()),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        "deckInfo" => {
            Some(object!{
                result: userdata["deck_info"].clone(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        "supporterAll" => {
            Some(object!{
                result: {
                    unit_support_list: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        "accessoryAll" => {
            Some(object!{
                result: {
                    accessory_list: [],
                    wearing_info: [],
                    especial_create_flag: false
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn costume(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "costumeList" => {
            Some(object!{
                result: {
                    costume_list: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn album(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn scenario(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "scenarioStatus" => {
            let scenario_list = json::parse(include_str!("../../assets/scenario_m.json")).unwrap();
            let mut scenario_status_list = array![];
            for (_i, data) in scenario_list.members().enumerate() {
                scenario_status_list.push(object!{
                    "scenario_id": data["scenario_id"].clone(),
                    "status": 1
                }).unwrap();
            }
            Some(object!{
                result: {
                    scenario_status_list: scenario_status_list
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn subscenario(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "subscenarioStatus" => {
            let subscenario_list = json::parse(include_str!("../../assets/card_navi_ids.json")).unwrap();
            let mut subscenario_status_list = array![];
            for (_i, data) in subscenario_list.members().enumerate() {
                subscenario_status_list.push(object!{
                    "subscenario_id": data.as_i32().unwrap(),
                    "status": 1
                }).unwrap();
            }
            Some(object!{
                result: {
                    subscenario_status_list: subscenario_status_list,
                    unlocked_multi_unit_scenario_ids: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn eventscenario(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn multiunit(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "multiunitscenarioStatus" => {
            let multi_unit_scenario_list = json::parse(include_str!("../../assets/multi_unit_scenario_open_m.json")).unwrap();
            let multi_unit_scenario_map = json::parse(include_str!("../../assets/multi_unit_scenario_map.json")).unwrap();
            let mut multi_unit_scenario_status_list = array![];
            for (_i, data) in multi_unit_scenario_list.members().enumerate() {
                multi_unit_scenario_status_list.push(object!{
                    "multi_unit_id": data["multi_unit_id"].clone(),
                    "status": 1,
                    "multi_unit_scenario_btn_asset": data["multi_unit_scenario_btn_asset"].clone(),
                    "open_date": data["open_date"].clone(),
                    "chapter_list": [
                        {
                            "multi_unit_scenario_id": multi_unit_scenario_map[data["multi_unit_id"].to_string()]["multi_unit_scenario_id"].clone(),
                            "chapter": 1,
                            "status": 1
                        }
                    ]
                }).unwrap();
            }
            Some(object!{
                result: {
                    multi_unit_scenario_status_list: multi_unit_scenario_status_list,
                    unlocked_subscenario_ids: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn payment(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "productList" => {
            Some(object!{
                result: {
                    restriction_info: {
                        restricted: false
                    },
                    under_age_info: {
                        birth_set: true,
                        has_limit: false,
                        limit_amount: null,
                        month_used: 0
                    },
                    sns_product_list: [],
                    product_list: [],
                    subscription_list: [],
                    show_point_shop: true
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn banner(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn notice(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "noticeMarquee" => {
            Some(object!{
                result: {
                    item_count: 0,
                    marquee_list: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn user(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "getNavi" => {
            Some(object!{
                result: {
                    user: {
                        user_id: userdata["user_info"]["user"]["user_id"].clone(),
                        unit_owning_user_id: 460498696 //TODO
                    }
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        "userInfo" => {
            Some(object!{
                result: userdata["user_info"].clone(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn navigation(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "specialCutin" => {
            Some(object!{
                result: {
                    special_cutin_list: []
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn award(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "awardInfo" => {
            let available_award_ranges = array![
                [1, 572],
                [1300, 1300],
                [10000, 10011],
                [999001, 999030]
            ];
            let available_award_blacklists = array![
                [455, 456, 457, 458, 475, 477, 478],
                [],
                [],
                [999007, 999015]
            ];
            let mut awards = array![];
            for (i, data) in available_award_ranges.members().enumerate() {
                for j in data[0].as_usize().unwrap()..data[1].as_usize().unwrap() {
                    if available_award_blacklists[j].contains(i) { continue; };
                    let is_set = if userdata["award_id"].as_usize().unwrap() == i { true } else { false };
                    awards.push(object!{
                        award_id: j,
                        is_set: is_set,
                        insert_date: global::timestamp_str()
                    }).unwrap();
                }
            }
            Some(object!{
                result: {
                    award_info: awards
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn background(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "backgroundInfo" => {
            let mut bginfo = array![];
            for i in 1..162 {
                let is_set = if userdata["bg_id"].to_string() == i.to_string() { true } else { false };
                bginfo.push(object!{
                    background_id: i,
                    is_set: is_set,
                    insert_date: global::timestamp_str()
                }).unwrap();
            }
            Some(object!{
                result: {
                    background_info: bginfo
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn stamp(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn exchange(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "owningPoint" => {
            Some(object!{
                result: {
                    //TODO - is this supposed to be static or not?
                    exchange_point_list: [
                        {
                            rarity: 2,
                            exchange_point: 2024
                        },
                        {
                            rarity: 3,
                            exchange_point: 167
                        },
                        {
                            rarity: 5,
                            exchange_point: 62
                        },
                        {
                            rarity: 10000,
                            exchange_point: 5038
                        },
                        {
                            rarity: 20002,
                            exchange_point: 10
                        }
                    ]
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn livese(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "liveseInfo" => {
            Some(object!{
                result: {
                    //TODO - is this supposed to be static or not?
                    live_se_list: [1, 2, 3]
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn liveicon(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "liveiconInfo" => {
            Some(object!{
                result: {
                    //TODO - is this supposed to be static or not?
                    live_notes_icon_list: [1, 2, 3]
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn item(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn marathon(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "marathonInfo" => {
            Some(object!{
                result: [],
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn challenge(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "challengeInfo" => {
            Some(object!{
                result: [],
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
fn login(userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "topInfo" => {
            Some(object!{
                result: global::login_result(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        "topInfoOnce" => {
            Some(object!{
                result: userdata["login_top_info"].clone(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
}
//todo
fn profile(_userdata: &JsonValue, _data: &JsonValue) -> Option<JsonValue> {
    None
}
fn secretbox(_userdata: &JsonValue, data: &JsonValue) -> Option<JsonValue> {
    match data["action"].to_string().as_str() {
        "all" => {
            let list = json::parse(include_str!("main-api/secretbox.all.result.json")).unwrap();
            Some(object!{
                result: {
                    use_cache: 0,
                    is_unit_max: true,
                    item_list: [],
                    gauge_info: {
                        max_gauge_point: 100,
                        gauge_point: 0
                    },
                    member_category_list: list["member_category_list"].clone()
                },
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            })
        }
        _ => None
    }
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
            resp_data.push(object!{
                result: json::parse(&STATIC_RESPONSE_DATA[index].to_string()).unwrap(),
                status: 200,
                commandNum: false,
                timeStamp: global::timestamp()
            }).unwrap();
            continue;
        }
        
        //messy
        match data["module"].as_str().unwrap() {
            "live" => { if let Some(d) = live(&userdata, data) { resp_data.push(d).unwrap(); } },
            "unit" => { if let Some(d) = unit(&userdata, data) { resp_data.push(d).unwrap(); } },
            "costume" => { if let Some(d) = costume(&userdata, data) { resp_data.push(d).unwrap(); } },
            "album" => { if let Some(d) = album(&userdata, data) { resp_data.push(d).unwrap(); } },
            "scenario" => { if let Some(d) = scenario(&userdata, data) { resp_data.push(d).unwrap(); } },
            "subscenario" => { if let Some(d) = subscenario(&userdata, data) { resp_data.push(d).unwrap(); } },
            "eventscenario" => { if let Some(d) = eventscenario(&userdata, data) { resp_data.push(d).unwrap(); } },
            "multiunit" => { if let Some(d) = multiunit(&userdata, data) { resp_data.push(d).unwrap(); } },
            "payment" => { if let Some(d) = payment(&userdata, data) { resp_data.push(d).unwrap(); } },
            "banner" => { if let Some(d) = banner(&userdata, data) { resp_data.push(d).unwrap(); } },
            "notice" => { if let Some(d) = notice(&userdata, data) { resp_data.push(d).unwrap(); } },
            "user" => { if let Some(d) = user(&userdata, data) { resp_data.push(d).unwrap(); } },
            "navigation" => { if let Some(d) = navigation(&userdata, data) { resp_data.push(d).unwrap(); } },
            "award" => { if let Some(d) = award(&userdata, data) { resp_data.push(d).unwrap(); } },
            "background" => { if let Some(d) = background(&userdata, data) { resp_data.push(d).unwrap(); } },
            "stamp" => { if let Some(d) = stamp(&userdata, data) { resp_data.push(d).unwrap(); } },
            "exchange" => { if let Some(d) = exchange(&userdata, data) { resp_data.push(d).unwrap(); } },
            "livese" => { if let Some(d) = livese(&userdata, data) { resp_data.push(d).unwrap(); } },
            "liveicon" => { if let Some(d) = liveicon(&userdata, data) { resp_data.push(d).unwrap(); } },
            "item" => { if let Some(d) = item(&userdata, data) { resp_data.push(d).unwrap(); } },
            "marathon" => { if let Some(d) = marathon(&userdata, data) { resp_data.push(d).unwrap(); } },
            "challenge" => { if let Some(d) = challenge(&userdata, data) { resp_data.push(d).unwrap(); } },
            "login" => { if let Some(d) = login(&userdata, data) { resp_data.push(d).unwrap(); } },
            "profile" => { if let Some(d) = profile(&userdata, data) { resp_data.push(d).unwrap(); } },
            "secretbox" => { if let Some(d) = secretbox(&userdata, data) { resp_data.push(d).unwrap(); } },
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
    let body = json::stringify(resp).replace("1680254953", &global::timestamp().to_string());
    global::sign(&req, &body).body(body)
}
