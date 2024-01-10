use redis::Commands;
use redis::Connection;
use std::error::Error;

struct RedisClient {
    client: redis::Client,
}

impl RedisClient {
    pub fn new() -> Result<RedisClient, Box<dyn Error>> {
        let client = redis::Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL

        todo!()
    }

    pub fn get_connection(&self) -> Result<redis::Connection, Box<dyn Error>> {
        let connection = self.client.get_connection().expect("Couldn't get connection.");
        Ok(connection)
    }
}

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

pub fn seq_push(conn: &mut Connection, key: &str, value: &str) -> Result<i64, Box<dyn Error>> {
    let length: i64 = conn.rpush(key, value)?;
    Ok(length)
}

pub fn seq_pop(conn: &mut Connection, key: &str) -> Result<Option<String>, Box<dyn Error>> {
    let value: Option<String> = conn.lpop(key, None)?;
    Ok(value)
}

pub fn seq_get_all(conn: &mut Connection, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let elements: Vec<String> = conn.lrange(key, 0, -1)?;
    Ok(elements)
}
