const logger = require("../logger")
const { Implication } = require("./predicate")
const assert = require("../assert")

function RandomWeight() {
    return (Math.random() - Math.random()) / 5
}

function PositiveFeature(feature) {
    return '++' + feature + '++'
}

function NegativeFeature(feature) {
    return '--' + feature + '--'
}

async function InitializeWeights(redis, implication) {
    assert.isType(implication, Implication)
    const weight1 = RandomWeight()
    const feature = implication.UniqueKey()
    await redis.client.hSet('weights', PositiveFeature(feature), weight1);
    const weight2 = RandomWeight()
    await redis.client.hSet('weights', NegativeFeature(feature), weight2);
}


async function ReadWeights(redis, features) {
    var r = {}
    for (const feature of Object.keys(features)) {
        const record = await redis.client.hGet('weights', feature);
        logger.dump({feature, record}, ReadWeights)
        r[feature] = parseFloat(record)
    }
    return r
}

async function SaveWeights(weights) {
    logger.noop({ weights }, SaveWeights)
    for (const feature of Object.keys(weights)) {
        const updated = await WeightRecord.findOneAndUpdate({ feature }, { weight: weights[feature] }, { new: true, runValidators: true });
        logger.noop({ feature, updated }, SaveWeights)
    }
}

async function DumpWeights() {
    // user = await client.hGetAll(userKey);
    logger.noop({}, DumpWeights)
}

module.exports = {PositiveFeature, NegativeFeature, InitializeWeights, ReadWeights, SaveWeights, DumpWeights}