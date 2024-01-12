use redis::Commands;
use redis::Connection;
use std::cell::RefCell;
use std::error::Error;

pub struct ConnectionFactory {
    client: redis::Client,
}

impl ConnectionFactory {
    pub fn new() -> Result<ConnectionFactory, Box<dyn Error>> {
        let client =
            redis::Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL
        let redis_client = ConnectionFactory { client };
        Ok(redis_client)
    }

    pub fn get_connection(&self) -> Result<RefCell<redis::Connection>, Box<dyn Error>> {
        let connection = self
            .client
            .get_connection()
            .expect("Couldn't get connection.");
        let refcell = RefCell::new(connection);
        Ok(refcell)
    }

    pub fn drop_all_dbs(&mut self) -> Result<(), Box<dyn Error>> {
        let connection = self.get_connection()?;
        redis::cmd("FLUSHDB").query(&mut connection.borrow_mut())?;
        trace!("Database flushed successfully");
        Ok(())
    }
}

pub fn map_insert(
    conn: &mut Connection,
    key: &str,
    field: &str,
    value: &str,
) -> Result<(), Box<dyn Error>> {
    conn.hset(key, field, value)?;
    Ok(())
}

pub fn map_get(
    conn: &mut Connection,
    key: &str,
    field: &str,
) -> Result<Option<String>, Box<dyn Error>> {
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
