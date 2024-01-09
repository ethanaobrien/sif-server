use rusqlite::{Connection, params};
use std::sync::Mutex;
use std::sync::MutexGuard;
use lazy_static::lazy_static;
use json::{object, JsonValue};
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
    conn.execute(
        "INSERT INTO uids (jsondata) VALUES (?1)",
        params!(json::stringify(existing_ids))
    ).unwrap();
    
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
        }
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
    let userdata = json::parse(&result.unwrap()).unwrap();
    
    return userdata;
    //return userdata["user_id"].as_i32().unwrap();
}
