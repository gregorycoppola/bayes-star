
async function StartRedis(dbName) {
    return new Storage();
}

class Redis {
    constructor() {

    }
}
async DropAllDBs() {
    const collections = await mongoose.connection.db.listCollections().toArray();
    for (const collection of collections) {
        await mongoose.connection.db.dropCollection(collection.name);
    }
    logger.noop('All collections dropped!', this.DropAllDBs);
}

module.exports = { StartRedis, ConnectDB }