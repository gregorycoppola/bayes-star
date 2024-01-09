use redis::Commands;
use redis::Connection;
use std::error::Error;

pub fn map_insert(conn: &mut Connection, key: &str, field: &str, value: &str) -> Result<(), Box<dyn Error>> {
    conn.hset(key, field, value)?;
    Ok(())
}

pub fn map_get(conn: &mut Connection, key: &str, field: &str) -> Result<Option<String>, Box<dyn Error>> {
    let value: Option<String> = conn.hget(key, field)?;
    Ok(value)
}

pub fn set_add(conn: &mut Connection, key: &str, member: &str) -> Result<bool, Box<dyn Error>> {
    let added: bool = conn.sadd(key, member)?;
    Ok(added)
}

pub fn set_members(conn: &mut Connection, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let members: Vec<String> = conn.smembers(key)?;
    Ok(members)
}
