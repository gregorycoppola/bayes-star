use redis::Connection;

pub struct BeliefPropagator {
    redis_connection: redis::Connection,
}

impl Drop for BeliefPropagator {
    fn drop(&mut self) {
        // The Drop trait for Arc<Client> will automatically be called here,
        // reducing the reference count. If this Storage instance holds the last
        // reference to the client, the client will be dropped and its resources
        // (like network connections) will be cleaned up.
    }
}

impl BeliefPropagator {
    // Initialize new Storage with a Redis connection
    pub fn new(connection: Connection) -> Result<Self, redis::RedisError> {
        Ok(BeliefPropagator {
            redis_connection: connection,
        })
    }
}