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

pub fn get_uid(device_info: &str) {
    let mut engine = ENGINE.lock().unwrap();
    if engine.is_none() { init(&mut engine); };
    
    
    
}
