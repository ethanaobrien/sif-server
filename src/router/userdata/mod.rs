use rusqlite::{Connection, params};
use std::sync::{Mutex, MutexGuard};
use lazy_static::lazy_static;
use json::{object, JsonValue, array};
use chrono::{Utc, DateTime};
use crate::router::global;

lazy_static! {
    pub static ref ENGINE: Mutex<Option<Connection>> = Mutex::new(None);
}

fn timestamp() -> String {
    let now: DateTime<Utc> = Utc::now();
    let formatted_date = now.format("%Y-%m-%d %H:%M:%S").to_string();
    return formatted_date;
}

fn init(engine: &mut MutexGuard<'_, Option<Connection>>) {
    let conn = Connection::open("userdata.db").unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", ()).unwrap();

    engine.replace(conn);
}
fn create_uid_store(conn: &Connection) {
    match conn.prepare("SELECT jsondata FROM uids") {
        Ok(_) => {}
        Err(_) => {
            conn.execute(
                "CREATE TABLE uids (
                    jsondata  TEXT NOT NULL
                )",
                (), // empty list of parameters.
            ).unwrap();
        }
    }
    store_data(conn, "uids", array![]);
}
fn acc_exists(conn: &Connection, key: &str) -> bool {
    match conn.prepare(&format!("SELECT jsondata FROM {}", key)) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}
fn store_data(conn: &Connection, key: &str, value: JsonValue) {
    conn.execute(
        &format!("INSERT INTO {} (jsondata) VALUES (?1)", key),
        params!(json::stringify(value))
    ).unwrap();
}

use rand::Rng;
fn get_uids(conn: &Connection) -> JsonValue {
    let mut stmt = conn.prepare("SELECT jsondata FROM uids").unwrap();
    let result: Result<String, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
    json::parse(&result.unwrap()).unwrap()
}

fn generate_uid(conn: &Connection) -> i32 {
    create_uid_store(conn);
    let mut rng = rand::thread_rng();
    let random_number = rng.gen_range(10_000_000..=99_999_999);
    let mut existing_ids = get_uids(conn);
    //the chances of this...?
    if existing_ids.contains(random_number) {
        return generate_uid(conn);
    }
    existing_ids.push(random_number).unwrap();
    store_data(conn, "uids", existing_ids);
    
    random_number
}

fn create_acc(conn: &Connection, key: &str) {
    let fully_random_uid = generate_uid(conn);
    conn.execute(
        &format!("CREATE TABLE {} (
            jsondata  TEXT NOT NULL
        )", key),
        (), // empty list of parameters.
    ).unwrap();
    
    store_data(conn, key, object!{
        user_id: fully_random_uid,
        user_data: {
            friend_action_cnt: 0,//1291,
            friend_greet_cnt: 0,
            friend_variety_cnt: 0,//1289,
            friend_new_cnt: 0,
            present_cnt: 0,
            secret_box_badge_flag: false,
            server_datetime: timestamp(),
            server_timestamp: global::timestamp(),
            notice_friend_datetime: "",
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
        },
        user_info: {
            "user": {
                "user_id": fully_random_uid,
                "name": "030",
                "level": 237,
                "exp": 779537,
                "previous_exp": 778427,
                "next_exp": 786041,
                "game_coin": 696903030,
                "sns_coin": 6903069,
                "free_sns_coin": 6903069,
                "paid_sns_coin": 0,
                "social_point": 696969,
                "unit_max": 9999,
                "waiting_unit_max": 9999,
                "energy_max": 6969,
                "energy_full_time": "2023-03-21 13:13:41",
                "license_live_energy_recoverly_time": 60,
                "energy_full_need_time": 0,
                "over_max_energy": 6969,
                "training_energy": 69,
                "training_energy_max": 69,
                "friend_max": 69,
                "invite_code": "258506983",
                "insert_date": "2018-05-01 19:28:30",
                "update_date": "2018-05-01 19:28:30",
                "tutorial_state": -1,
                "lp_recovery_item": []
            },
            "ad_status": true,
            "server_timestamp": ""
        },
        live_status: json::parse(include_str!("../../../assets/liveStatus.json")).unwrap(),
        unit_all: json::parse(include_str!("../../../assets/unitAll.json")).unwrap(),
        deck_info: [
            {
                "unit_deck_id": 1,
                "main_flag": true,
                "deck_name": "ユニットA",
                "unit_owning_user_ids": [
                    {
                        "position": 1,
                        "unit_owning_user_id": 460498708
                    },
                    {
                        "position": 2,
                        "unit_owning_user_id": 460498704
                    },
                    {
                        "position": 3,
                        "unit_owning_user_id": 460498703
                    },
                    {
                        "position": 4,
                        "unit_owning_user_id": 460498718
                    },
                    {
                        "position": 5,
                        "unit_owning_user_id": 460500692
                    },
                    {
                        "position": 6,
                        "unit_owning_user_id": 460498719
                    },
                    {
                        "position": 7,
                        "unit_owning_user_id": 460498716
                    },
                    {
                        "position": 8,
                        "unit_owning_user_id": 460498715
                    },
                    {
                        "position": 9,
                        "unit_owning_user_id": 460498714
                    }
                ]
            }
        ],
        login_top_info: {
            new_achievement_cnt: 0,
            unaccomplished_achievement_cnt: 0,
            live_daily_reward_exist: false,
            training_energy: 69,
            training_energy_max: 69,
            notification: {
                push: false,
                lp: false,
                update_info: false,
                campaign: false,
                live: false,
                lbonus: false,
                event: false,
                secretbox: false,
                birthday: false
            },
            open_arena: false,// true,
            costume_status: false,
            open_accessory: false,
            arena_si_skill_unique_check: false, // true,
            open_v98: false
        },
        bg_id: 149,
        award_id: 10011
    });
}

pub fn get_acc(device_info: &str) -> JsonValue {
    let key = &format!("_{}_", device_info[0..20].to_lowercase());
    let mut engine = ENGINE.lock().unwrap();
    if engine.is_none() { init(&mut engine); };
    let conn = engine.as_ref().unwrap();
    if !acc_exists(conn, key) {
        create_acc(conn, key);
    }
    let mut stmt = conn.prepare(&format!("SELECT jsondata FROM {}", key)).unwrap();
    let result: Result<String, rusqlite::Error> = stmt.query_row([], |row| row.get(0));
    let mut userdata = json::parse(&result.unwrap()).unwrap();
    userdata["user_info"]["server_timestamp"] = json::JsonValue::Number(global::timestamp().into());
    
    return userdata;
    //return userdata["user_id"].as_i32().unwrap();
}

pub fn save_acc(device_info: &str, data: JsonValue) {
    let key = &format!("_{}_", device_info[0..20].to_lowercase());
    let mut engine = ENGINE.lock().unwrap();
    if engine.is_none() { init(&mut engine); };
    let conn = engine.as_ref().unwrap();
    if !acc_exists(conn, key) {
        create_acc(conn, key);
    }
    store_data(conn, key, data);
}

pub fn get_unitall(data: JsonValue) -> JsonValue{
    let mut rv = object!{
        active: [],
        waiting: []
    };
    
    for (_i, person) in data["active"].entries().enumerate() {
        rv["active"].push(person.1.clone()).unwrap();
    }
    for (_i, person) in data["waiting"].entries().enumerate() {
        rv["waiting"].push(person.1.clone()).unwrap();
    }
    return rv;
}
