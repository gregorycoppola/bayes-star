const logger = require("../logger")
const { CreateStorage } = require("./storage")
const { CreateRedisClient } = require("./redis")
const { ComputeBacklinks } = require("./choose")
const { InitializeWeights, TrainOnExample } = require("./maxent")
const { DumpWeights } = require("./weights")

async function main() {
    logger.dump({starting: "training"}, main)
    const redis = await CreateRedisClient()
    const storage = await CreateStorage(redis)
    const implications = await storage.GetAllImplications()
    for (const implication of implications) {
        logger.dump({ implication }, main)
        await InitializeWeights(redis, implication)
    }

    const propositions = await storage.GetAllPropositions()
    for (const proposition of propositions) {
        logger.dump({ proposition }, main)
        const backlinks = await ComputeBacklinks(storage, proposition)
        await TrainOnExample(storage, proposition, backlinks)
    }
    await DumpWeights(redis)
    await redis.Disconnect();
}

main()