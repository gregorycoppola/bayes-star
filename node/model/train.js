const mongoose = require("mongoose")
const logger = require("../logger")
const { StartRedis, ConnectDB } = require("./storage2")
const { ComputeBacklinks } = require("./choose")
const { InitializeWeights, TrainOnExample, DumpWeights } = require("./maxent")

async function main() {
    const storage = await StartRedis()
    const implications = await storage.GetAllImplications()
    for (const implication of implications) {
        logger.noop({ implication }, main)
        await InitializeWeights(implication)
    }

    const propositions = await storage.GetAllPropositions()
    logger.noop({ propositions }, main)
    for (const proposition of propositions) {
        const backlinks = await ComputeBacklinks(storage, proposition)
        await TrainOnExample(storage, proposition, backlinks)
    }
    await DumpWeights()
    await storage.Disconnect();
}

main()