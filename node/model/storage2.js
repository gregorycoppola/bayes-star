const mongoose = require("mongoose")
const assert = require("../assert");
const logger = require("../logger")
const redis = require("redis")
const { promisify } = require('util');
const { PropositionRecord, ImplicationRecord, EntityRecord } = require("./models")
const { Proposition, Implication, Entity } = require("./predicate")

class Storage {
    constructor() {
        const client = redis.createClient({
            url: 'redis://localhost:6379'
        });

        client.on('error', (err) => console.log('Redis Client Error', err));

        this.client = client
    }

    async Connect() {
        await this.client.connect();
    }

    async DropAllDBs() {
        assert.isTrue(this.client)
        const flushResult = await this.client.flushDb();
        console.log({ flushResult });
    }

    async StoreEntity(entity) {
        // map from key to list of strings... 
        assert.isType(entity, Entity)
        await this.client.sAdd(entity.domain, entity.name);
    }

    async GetEntitiesInDomain(domain) {
        let set1Members = await this.client.sMembers(domain);
        logger.dump('Members of set1:', set1Members);
        return set1Members.map(name => new Entity(domain, name))
    }

    async StoreProposition(proposition, probability) {
        assert.isType(proposition, Proposition)
        assert.isTrue(proposition.IsFact())
        const searchString = proposition.SearchString();
        const record = proposition.ToString();
        await this.client.hSet('propositions', searchString, record);
    }

    async GetAllPropositions() {
        const allValues = await this.client.hGetAll('propositions');
        for (const [key, value] of Object.entries(allValues)) {
            console.log(`Key: ${key}, Value: ${value}`);
        }
        process.exit() // TODO: implement this
    }

    async StoreImplication(implication) {
        assert.isType(implication, Implication)
        logger.noop({ implication }, this.StoreImplication)
        const searchString = implication.SearchString();
        const record = implication.ToString();
        logger.dump({searchString, record}, this.StoreImplication)
        await this.client.hSet('implications', searchString, record);
    }

    async GetAllImplications() {
        const allValues = await this.client.hGetAll('propositions');
        for (const [key, value] of Object.entries(allValues)) {
            console.log(`Key: ${key}, Value: ${value}`);
        }
        process.exit() // TODO: implement this
    }

    async FindPremises(searchString) {
        assert.isType(searchString, "string")
        logger.noop({ searchString }, this.FindPremises)
        logger.noop({ searchString }, this.FindPremises)
        const rows = await ImplicationRecord.find({ searchString });
        var result = []
        for (const row of rows) {
            result.push(Implication.FromString(row))
        }
        logger.noop({ result }, this.FindPremises)
        return result;
    }

    async Disconnect() {
        await this.client.quit();
    }
}

async function CreateStorage(dbName) {
    return new Storage();
}

async function ConnectDB() {
    const storage = new Storage()
    await storage.Connect()
    return storage
}


module.exports = { Storage, CreateStorage, ConnectDB }