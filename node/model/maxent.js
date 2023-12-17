const logger = require("../logger")
const assert = require("../assert")
const { ComputeBacklinks } = require("./choose")
const {PositiveFeature, NegativeFeature, InitializeWeights, ReadWeights, SaveWeights, DumpWeights} = require("./weights")

function Sigmoid(x) {
    return 1 / (1 + Math.exp(-x));
}

function DotProduct(dict1, dict2) {
    logger.noop({ dict1, dict2 }, DotProduct)
    let result = 0;
    for (const key of Object.keys(dict1)) {
        const v1 = dict1[key]
        const v2 = dict2[key]
        if (v1 == null || v2 == null) {
            logger.noop({ null_: true, v1, v2, result }, DotProduct)
        } else {
            result += v1 * v2
            logger.noop({ null_: false, v1, v2, result }, DotProduct)
        }
    }
    return result;
}

async function FeaturesFromBacklinks(storage, backlinks) {
    var result = {}
    for (var i = 0; i < backlinks.length; i++) {
        const backlink = backlinks[i]
        logger.noop({ backlink }, FeaturesFromBacklinks)
        const feature = backlink.implication.UniqueKey()
        const probability = await storage.GetPropositionProbability(backlink.proposition)
        const posf = PositiveFeature(feature)
        const negf = NegativeFeature(feature)
        logger.noop({ feature,  probability, posf, negf }, FeaturesFromBacklinks)
        result[posf] = probability
        result[negf] = 1 - probability
    }
    return result
}

function ComputeProbability(weights, features) {
    logger.noop({ weights, features }, ComputeProbability)
    const dot = DotProduct(weights, features);
    const probability = Sigmoid(dot);
    return probability;
}

function ComputeExpectedFeatures(probability, features) {
    logger.noop({ probability, features }, ComputeExpectedFeatures)
    let r = {};
    for (let key in features) {
        r[key] = features[key] * probability;
    }
    logger.noop({ r }, ComputeExpectedFeatures)
    return r;
}

const LEARNING_RATE = 0.1

function DoSGDUpdate(weights, goldFeatures, expectedFeatures) {
    logger.noop({weights, goldFeatures, expectedFeatures}, DoSGDUpdate)
    var r = {}
    for (const feature of Object.keys(weights)) {
        const wv = weights[feature]
        const gv = goldFeatures[feature]
        const ev = expectedFeatures[feature]
        assert.isTrue(wv != null)
        assert.isTrue(gv != null)
        assert.isTrue(ev != null)
        const newWeight = wv + LEARNING_RATE * (gv - ev)
        const loss = Math.abs(gv - ev)
        logger.dump({ feature, wv, gv, ev, loss, newWeight }, DoSGDUpdate)
        r[feature] = newWeight
    }
    return r
}

async function TrainOnExample(storage, proposition, backlinks) {
    logger.noop({ proposition, backlinks }, TrainOnExample)

    const features = await FeaturesFromBacklinks(storage, backlinks)
    logger.noop({ proposition, features }, TrainOnExample)

    const weightVector = await ReadWeights(storage.redis, features)
    logger.noop({ features, weightVector }, TrainOnExample)

    const probability = ComputeProbability(weightVector, features);
    logger.noop({ proposition, probability }, TrainOnExample)

    const expected = ComputeExpectedFeatures(probability, features)
    logger.noop({ expected }, TrainOnExample)

    const newWeight = DoSGDUpdate(weightVector, features, expected)
    logger.noop({ weightVector, newWeight }, TrainOnExample)

    await SaveWeights(storage.redis, newWeight)
}

async function DoTraining(storage) {
    const redis = storage.redis
    logger.noop({starting: "training"}, DoTraining)
    const implications = await storage.GetAllImplications()
    for (const implication of implications) {
        logger.noop({ implication }, DoTraining)
        await InitializeWeights(redis, implication)
    }

    const propositions = await storage.GetAllPropositions()
    for (const proposition of propositions) {
        logger.noop({ proposition }, DoTraining)
        const backlinks = await ComputeBacklinks(storage, proposition)
        await TrainOnExample(storage, proposition, backlinks)
    }
    await DumpWeights(redis)
}


module.exports = { InitializeWeights, TrainOnExample, DoTraining }