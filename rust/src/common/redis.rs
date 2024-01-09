use redis::Commands;
use redis::Connection;
use std::error::Error;

pub fn insert_into_map(conn: &mut Connection, key: &str, field: &str, value: &str) -> Result<(), Box<dyn Error>> {
    conn.hset(key, field, value)?;
    Ok(())
}


pub fn read_from_map(conn: &mut Connection, key: &str, field: &str) -> Result<Option<String>, Box<dyn Error>> {
    let value: Option<String> = conn.hget(key, field)?;
    Ok(value)
}
