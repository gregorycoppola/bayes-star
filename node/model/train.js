const mongoose = require("mongoose")
const logger = require("../logger")
const { CreateStorage, ConnectDB } = require("./storage2")
const { ComputeBacklinks } = require("./choose")
const { InitializeWeights, TrainOnExample, DumpWeights } = require("./maxent")

async function main() {
    await ConnectDB()
    const storage = await CreateStorage("testdb1")
    const implications = await storage.GetAllImplications()
    for (const implication of implications) {
        logger.noop({ implication }, main)
        await InitializeWeights(implication)
    }

    const propositions = await storage.GetAllPropositions()
    logger.noop({ propositions }, main)
    for (const proposition of propositions) {
        const backlinks = await ComputeBacklinks(storage, proposition)
        await TrainOnExample(proposition, backlinks)
    }
    await DumpWeights()
    await mongoose.disconnect();
}

main()