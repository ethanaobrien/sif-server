use rusqlite::{Connection};
use std::sync::Mutex;
use std::sync::MutexGuard;
use lazy_static::lazy_static;

lazy_static! {
    pub static ref ENGINE: Mutex<Option<Connection>> = Mutex::new(None);
}

pub fn init(engine: &mut MutexGuard<'_, Option<Connection>>) {
    let conn = Connection::open("userdata.db").unwrap();
    conn.execute("PRAGMA foreign_keys = ON;", ()).unwrap();

    engine.replace(conn);
}
pub fn acc_exists(conn: &Connection, key: &str) -> bool {
    match conn.prepare(&format!("SELECT jsondata FROM {}", key)) {
        Ok(_) => {
            true
        }
        Err(_) => {
            false
        }
    }
}
pub fn create_acc(conn: &Connection, key: &str) {
    let fully_random_uid = 69696969;
    conn.execute(
        &format!("CREATE TABLE {} (
            jsondata  TEXT NOT NULL
        )", key),
        (), // empty list of parameters.
    ).unwrap();
    //conn.execute(
    //    &format!("INSERT INTO {} (jsondata) VALUES (?1)", key),
    //    "asd",
    //).unwrap();
    
}

pub fn get_uid(device_info: &str) -> i32 {
    let mut engine = ENGINE.lock().unwrap();
    if engine.is_none() { init(&mut engine); };
    let conn = engine.as_ref().unwrap();
    if !acc_exists(conn, device_info) {
        create_acc(conn, device_info);
    }
    let stmt = conn.execute(&format!("SELECT jsondata FROM {}", device_info), ()).unwrap();
    println!("{}", stmt);
    
    
    
    return 100101;
    
    
}
