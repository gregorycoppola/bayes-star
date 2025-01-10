use redis::Commands;
use redis::Connection;
use std::cell::RefCell;
use std::error::Error;
use std::sync::Arc;
use std::sync::Mutex;

pub struct RedisManager {
    client: redis::Client,
}

impl RedisManager {
    pub fn new() -> Result<RedisManager, Box<dyn Error>> {
        let client =
            redis::Client::open("redis://127.0.0.1/").expect("Could not connect to Redis."); // Replace with your Redis server URL
        let redis_client = RedisManager { client };
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

    pub fn get_mutex_guarded_connection(&self) -> Result<Mutex<redis::Connection>, Box<dyn Error>> {
        let connection = self
            .client
            .get_connection()
            .expect("Couldn't get connection.");
        let refcell = Mutex::new(connection);
        Ok(refcell)
    }

    pub fn get_arc_mutex_guarded_connection(&self) -> Result<Arc<Mutex<redis::Connection>>, Box<dyn Error>> {
        let connection = self
            .client
            .get_connection()
            .expect("Couldn't get connection.");
        let refcell = Arc::new(Mutex::new(connection));
        Ok(refcell)
    }
}

fn namespace_qualified_key(namespace: &str, key: &str) -> String {
    format!("bayes-star:{namespace}:{key}")
}

pub fn set_value(
    conn: &mut Connection,
    namespace: &str,
    key: &str,
    value: &str,
) -> Result<(), Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    conn.set(nskey, value)?;
    Ok(())
}

pub fn get_value(
    conn: &mut Connection,
    namespace: &str,
    key: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let value: Option<String> = conn.get(nskey)?;
    trace!("nskey: {nskey}, value: {:?}", &value);
    Ok(value)
}

pub fn map_insert(
    conn: &mut Connection,
    namespace: &str,
    key: &str,
    field: &str,
    value: &str,
) -> Result<(), Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    conn.hset(nskey, field, value)?;
    Ok(())
}

pub fn map_get(
    conn: &mut Connection,
    namespace: &str,
    key: &str,
    field: &str,
) -> Result<Option<String>, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let value: Option<String> = conn.hget(nskey, field)?;
    Ok(value)
}

pub fn set_add(conn: &mut Connection, namespace: &str, key: &str, member: &str) -> Result<bool, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let added: bool = conn.sadd(nskey, member)?;
    Ok(added)
}

pub fn set_members(conn: &mut Connection, namespace: &str, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let members: Vec<String> = conn.smembers(nskey)?;
    Ok(members)
}

pub fn is_member(conn: &mut Connection, namespace: &str, key: &str, member: &str) -> Result<bool, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let is_member: bool = conn.sismember(nskey, member)?;
    Ok(is_member)
}

pub fn seq_push(conn: &mut Connection, namespace: &str, key: &str, value: &str) -> Result<i64, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let length: i64 = conn.rpush(nskey, value)?;
    Ok(length)
}

// pub fn seq_pop(conn: &mut Connection, key: &str) -> Result<Option<String>, Box<dyn Error>> {
//     let value: Option<String> = conn.lpop(key, None)?;
//     Ok(value)
// }

pub fn seq_get_all(conn: &mut Connection, namespace: &str, key: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let nskey = &namespace_qualified_key(namespace, key);
    let elements: Vec<String> = conn.lrange(nskey, 0, -1)?;
    Ok(elements)
}
