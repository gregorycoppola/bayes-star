const assert = require("../assert");
const logger = require("../logger")

const redis = require("redis")
class RedisClient {
    constructor() {
        const client = redis.createClient({
            url: 'redis://localhost:6379'
        });
        client.on('error', (err) => console.log('Redis Client Error', err));
        this.client = client
    }

    async Disconnect() {
        await this.client.quit();
    }

    async Connect() {
        await this.client.connect();
    }

    async DropAllDBs() {
        assert.isTrue(this.client)
        const flushResult = await this.client.flushDb();
        logger.dump({ flushResult }, this.DropAllDBs);
    }
}

async function CreateRedisClient() {
    const client = new RedisClient();
    await client.Connect();
    return client
}

module.exports = { CreateRedisClient, RedisClient }