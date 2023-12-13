const mongoose = require("mongoose")
const logger = require("../logger")
const { Implication } = require("./predicate")
const models = require("./models")
const assert = require("../assert")

const weightSchema = new mongoose.Schema({
    feature: { type: String, required: true },
    weight: { type: Number, required: true },
})
const WeightRecord = mongoose.model('Weight', weightSchema);

function RandomWeight() {
    return (Math.random() - Math.random()) / 5
}

function PositiveFeature(feature) {
    return '+++' + feature + '+++'
}

function NegativeFeature(feature) {
    return '---' + feature + '---'
}

async function InitializeWeights(implication) {
    assert.isType(implication, Implication)
    const weight = RandomWeight()
    const feature = implication.UniqueKey()
    const pwr = WeightRecord({ feature: PositiveFeature(feature), weight })
    await pwr.save()
    const nwr = WeightRecord({ feature: NegativeFeature(feature), weight })
    await nwr.save()
    logger.noop({ pwr, nwr }, InitializeWeights)
}

async function GetPropositionProbability(searchString) {
    assert.isType(searchString, "string")
    logger.dump({searchString}, GetPropositionProbability)
    const record = await models.PropositionRecord.findOne({ searchString })
    const r = record.probability
    logger.noop({ searchString, record, r }, GetPropositionProbability)
    return r
}

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

async function FeaturesFromBacklinks(backlinks) {
    var result = {}
    for (var i = 0; i < backlinks.length; i++) {
        const backlink = backlinks[i]
        logger.noop({ backlink }, FeaturesFromBacklinks)
        const feature = backlink.implication.UniqueKey()
        const searchString = backlink.proposition.SearchString()
        const probability = await GetPropositionProbability(searchString)
        logger.noop({ feature, searchString, probability }, FeaturesFromBacklinks)
        result[PositiveFeature(feature)] = probability
        result[NegativeFeature(feature)] = 1 - probability
    }
    return result
}

function ComputeProbability(weights, features) {
    logger.noop({ weights, features }, ComputeProbability)
    const dot = DotProduct(weights, features);
    const probability = Sigmoid(dot);
    return probability;
}

async function ReadWeights(features) {
    logger.noop({ features }, ReadWeights)
    var r = {}
    for (const feature of Object.keys(features)) {
        const record = await WeightRecord.findOne({ feature })
        logger.noop({ feature, record }, ReadWeights)
        if (record) {
            r[feature] = record.weight
        }
    }
    logger.noop({ r }, ReadWeights)
    return r
}

async function SaveWeights(weights) {
    logger.noop({ weights }, SaveWeights)
    for (const feature of Object.keys(weights)) {
        const updated = await WeightRecord.findOneAndUpdate({ feature }, { weight: weights[feature] }, { new: true, runValidators: true });
        logger.noop({ feature, updated }, SaveWeights)
    }
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
    var r = {}
    for (const feature of Object.keys(weights)) {
        const wv = weights[feature]
        const gv = goldFeatures[feature]
        const ev = expectedFeatures[feature]
        assert.isTrue(wv != null)
        assert.isTrue(gv != null)
        assert.isTrue(ev != null)
        const newWeight = wv + LEARNING_RATE * (gv - ev)
        logger.noop({ feature, wv, gv, ev, newWeight }, DoSGDUpdate)
        r[feature] = newWeight
    }
    return r
}

async function TrainOnExample(proposition, backlinks) {
    logger.noop({ proposition, backlinks }, TrainOnExample)

    const features = await FeaturesFromBacklinks(backlinks)
    logger.noop({ proposition, features }, TrainOnExample)

    const weightVector = await ReadWeights(features)
    logger.noop({ features, weightVector }, TrainOnExample)

    const probability = ComputeProbability(weightVector, features);
    logger.noop({ probability }, TrainOnExample)

    const expected = ComputeExpectedFeatures(probability, features)
    logger.noop({ expected }, TrainOnExample)

    const newWeight = DoSGDUpdate(weightVector, features, expected)
    logger.noop({ weightVector, newWeight }, TrainOnExample)

    await SaveWeights(newWeight)
}

async function DumpWeights() {
    const records = await WeightRecord.find({})
    logger.noop({ records }, DumpWeights)
}

module.exports = { InitializeWeights, TrainOnExample, DumpWeights }