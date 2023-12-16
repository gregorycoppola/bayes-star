const assert = require("../assert");
const logger = require("../logger")
const { Proposition, Implication, Entity } = require("./predicate")

class Storage {
    constructor(redis) {
        this.redis = redis
    }

    async StoreEntity(entity) {
        assert.isType(entity, Entity)
        await this.redis.client.sAdd(entity.domain, entity.name);
    }

    async GetEntitiesInDomain(domain) {
        let set1Members = await this.redis.client.sMembers(domain);
        logger.noop('Members of set1:', set1Members);
        return set1Members.map(name => new Entity(domain, name))
    }

    async StoreProposition(proposition, probability) {
        assert.isType(proposition, Proposition)
        assert.isTrue(proposition.IsFact())
        const searchString = proposition.SearchString();
        const record = proposition.ToString();
        await this.redis.client.hSet('propositions', searchString, record);
    }

    async GetAllPropositions() {
        const allValues = await this.redis.client.hGetAll('propositions');
        for (const [key, value] of Object.entries(allValues)) {
            logger.dump({key, value}, this.GetAllPropositions)
        }
        process.exit() // TODO: implement this
    }

    async StoreImplication(implication) {
        assert.isType(implication, Implication)
        logger.noop({ implication }, this.StoreImplication)
        const searchString = implication.SearchString();
        const record = implication.ToString();
        logger.noop({searchString, record}, this.StoreImplication)
        await this.redis.client.hSet('implications', searchString, record);
    }

    async GetAllImplications() {
        const allValues = await this.redis.client.hGetAll('implications');
        var r = []
        for (const [key, value] of Object.entries(allValues)) {
            logger.dump({key, value}, this.GetAllImplications)
            const implication = Implication.FromString(value)
            logger.dump({implication}, this.GetAllImplications)
            r.push(implication)
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
}

async function CreateStorage(redis) {
    return new Storage(redis)
}

module.exports = { CreateStorage }