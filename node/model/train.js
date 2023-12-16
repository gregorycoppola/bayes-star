const logger = require("../logger")
const { CreateStorage } = require("./storage")
const { CreateRedisClient } = require("./redis")
const { ComputeBacklinks } = require("./choose")
const { InitializeWeights, TrainOnExample, DumpWeights } = require("./maxent")

async function main() {
    const redis = await CreateRedisClient()
    const storage = await CreateStorage(redis)
    const implications = await storage.GetAllImplications()
    for (const implication of implications) {
        logger.dump({ implication }, main)
        await InitializeWeights(implication)
    }

    const propositions = await storage.GetAllPropositions()
    for (const proposition of propositions) {
        logger.noop({ proposition }, main)
        const backlinks = await ComputeBacklinks(storage, proposition)
        await TrainOnExample(storage, proposition, backlinks)
    }
    await DumpWeights()
    await storage.Disconnect();
}

main()