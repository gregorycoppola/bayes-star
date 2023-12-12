const mongoose = require("mongoose")
const assert = require("../assert");
const logger = require("../logger")
const {PropositionRecord, ImplicationRecord, EntityRecord} = require("./models")
const { Proposition, Implication, Entity } = require("./predicate")

class Storage {
    constructor(mongoUrl) {
        this.mongoUrl = mongoUrl
    }

    async DropAllDBs() {
        const collections = await mongoose.connection.db.listCollections().toArray();
        for (const collection of collections) {
            await mongoose.connection.db.dropCollection(collection.name);
        }
        logger.noop('All collections dropped!', this.DropAllDBs);
    }

    async StoreEntity(entity) {
        assert.isType(entity, Entity)
        const newItem = new EntityRecord({ name: entity.name, domain: entity.domain });
        await newItem.save();
    }
    async GetEntitiesInDomain(domain) {
        const results = await EntityRecord.find({ domain });
        var buffer = []
        for (const record of results) {
            buffer.push(new Entity(record.domain, record.name))
        }
        return buffer
    }

    async GetAllPropositions() {
        const results = await PropositionRecord.find({ });
        logger.noop({results}, this.GetAllPropositions)
        var buffer = []
        for (const result of results) {
            const proposition = Proposition.FromString(result.record)
            logger.noop({pushing:1, proposition}, this.GetAllPropositions)

            buffer.push(proposition)
        }
        logger.noop({size: buffer.length}, this.GetAllPropositions)
        return buffer
    }

    async GetAllImplications() {
        const results = await ImplicationRecord.find({ });
        var buffer = []
        for (const record of results) {
            buffer.push(Implication.FromRecord(record))
        }
        return buffer
    }

    async StoreProposition(proposition, probability) {
        assert.isType(proposition, Proposition)
        assert.isTrue(proposition.IsFact())
        const searchString = proposition.SearchString();
        const record = proposition.ToString();
        logger.noop({ searchString, record })
        const updatedEntry = await PropositionRecord.findOneAndUpdate(
            { searchString },
            {
                searchString,
                record,
                probability,
            },
            { new: true, upsert: true }
        );

        logger.noop(`Document inserted/updated: ${updatedEntry}`, this.StoreProposition);
        return updatedEntry;
    }

    async StoreImplication(implication) {
        assert.isType(implication, Implication)
        logger.noop({ implication }, this.StoreImplication)
        const searchString = implication.SearchString();
        const UniqueKey = implication.UniqueKey();
        const updatedEntry = await ImplicationRecord.findOneAndUpdate(
            { UniqueKey },
            {
                UniqueKey,
                searchString,
                premiseRecord: implication.premise.ToString(),
                conclusionRecord: implication.conclusion.ToString(),
                mappingRecord: implication.MappingString(),
                featureString: implication.FeatureString()
            },
            { new: true, upsert: true }
        );
        logger.noop(`Document inserted/updated: ${updatedEntry}`, this.StoreImplication);
        return updatedEntry;
    }

    async FindPremises(searchString) {
        assert.isType(searchString, "string")
        logger.noop({searchString}, this.FindPremises)
        logger.noop({ searchString}, this.FindPremises)
        const rows = await ImplicationRecord.find({ searchString });
        var result = []
        for (const row of rows) {
            result.push(Implication.FromRecord(row))
        }
        logger.noop({ result }, this.FindPremises)
        return result;
    }
}

async function CreateStorage(dbName) {
    return new Storage();
}

async function ConnectDB() {
    const baseUrl = 'mongodb://localhost:27017';
    const dbName = 'testdb1'
    const url = baseUrl + '/' + dbName
    await mongoose.connect(url, { useNewUrlParser: true, useUnifiedTopology: true });
    logger.noop('Connected successfully to server', ConnectDB);
}


module.exports = { Storage, CreateStorage, ConnectDB }