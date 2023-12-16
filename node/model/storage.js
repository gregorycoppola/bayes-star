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
        const record = JSON.stringify(proposition);
        logger.noop({ record }, this.StoreProposition)
        const recovered = JSON.parse(record)
        logger.noop({ recovered }, this.StoreProposition)
        await this.redis.client.hSet('propositions', searchString, record);
        await this.StorePropositionProbability(proposition, probability)
    }

    async GetAllPropositions() {
        const allValues = await this.redis.client.hGetAll('propositions');
        logger.noop({ allValues }, this.GetAllPropositions)
        var r = []
        for (const [key, value] of Object.entries(allValues)) {
            logger.noop({ key, value }, this.GetAllPropositions)
            const tuple = JSON.parse(value)
            r.push(Proposition.FromTuple(tuple))
        }
        return r
    }

    async StorePropositionProbability(proposition, probability) {
        assert.isType(proposition, Proposition)
        assert.isTrue(proposition.IsFact())
        const searchString = proposition.SearchString();
        logger.noop({ searchString, probability }, this.StorePropositionProbability)
        await this.redis.client.hSet('probs', searchString, probability);
    }
    

    async GetPropositionProbability(proposition) {
        assert.isType(proposition, Proposition)
        assert.isTrue(proposition.IsFact())
        const searchString = proposition.SearchString();
        const r = this.redis.client.hGet('probs', searchString);

        function parseOrUseDefault(value, defaultValue) {
            const parsedValue = parseFloat(value);
            if (isNaN(parsedValue)) {
                console.log('fail', searchString)
                return defaultValue;
            } else {
                console.log('good', searchString)
            }
            return parsedValue;
        }

        const rv = parseOrUseDefault(r, 0.5)
        logger.noop({searchString, r, rv}, this.GetPropositionProbability)
        return rv
    }

    async StoreImplication(implication) {
        assert.isType(implication, Implication)
        logger.noop({ implication }, this.StoreImplication)
        const searchString = implication.SearchString();
        const record = JSON.stringify(implication)
        logger.noop({ searchString, record }, this.StoreImplication)
        // const recovered = JSON.parse(record)
        // logger.noop({ recovered }, this.StoreImplication)
        await this.redis.client.sAdd('implications', record);
        await this.StoreLinks(implication)
    }

    async GetAllImplications() {
        const allValues = await this.redis.client.sMembers('implications');
        logger.noop({ allValues }, this.GetAllImplications)
        var r = []
        for (const record of allValues) {
            logger.noop({ record }, this.GetAllImplications)
            const implication = Implication.FromTuple(JSON.parse(record))
            logger.noop({ implication }, this.GetAllImplications)
            r.push(implication)
        }
        return r
    }

    async StoreLinks(implication) {
        assert.isType(implication, Implication)
        logger.noop({ implication }, this.StoreImplication)
        const searchString = implication.SearchString();
        const record = JSON.stringify(implication)
        await this.redis.client.sAdd(searchString, record);
    }

    async FindPremises(searchString) {
        assert.isType(searchString, "string")
        let set1Members = await this.redis.client.sMembers(searchString);
        logger.noop({ searchString, set1Members }, this.FindPremises);
        var r = []
        for (const record of set1Members) {
            const tuple = JSON.parse(record)
            logger.noop({ record, tuple }, this.FindPremises)
            const implication = Implication.FromTuple(tuple)
            r.push(implication)
        }
        return r
    }
}

async function CreateStorage(redis) {
    return new Storage(redis)
}

module.exports = { CreateStorage, Storage }