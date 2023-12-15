const { Proposition, BackLink } = require("./predicate");
const assert = require("../assert");
const logger = require("../logger");
const { Storage } = require("./storage2");
const { ConvertToQuantified, ExtractPremiseRoleMap, ConvertToProposition } = require("./ops");

function Combine(inputArray, k) {
    var result = [];
    function run(level, start, currentArray) {
        if (currentArray.length === k) {
            result.push(currentArray.slice());
            return;
        }
        for (var i = start; i < inputArray.length; i++) {
            currentArray.push(inputArray[i]);
            run(level + 1, i + 1, currentArray);
            currentArray.pop();
        }
    }
    run(0, 0, []);
    return result;
}

function ComputeChooseConfigurations(N, K) {
    var inputArray = Array.from({ length: N }, (_, i) => i);
    return Combine(inputArray, K);
}

function ExtractRolesFromIndices(roles, indices) {
    var result = []
    const indexSet = new Set(indices)
    for (var i = 0; i < roles.length; i++) {
        if (indexSet.has(i)) {
            result.push(roles[i])
            logger.noop({ has: true, i, indexSet }, ExtractRolesFromIndices)
        }
    }
    return result
}

function ComputeSearchKeys(proposition) {
    assert.isType(proposition, Proposition)
    assert.isTrue(proposition.IsFact())
    const num_roles = proposition.roles.length
    const configurations1 = ComputeChooseConfigurations(num_roles, 1)
    const configurations2 = ComputeChooseConfigurations(num_roles, 2)
    logger.noop({ configurations1, configurations2 }, 'ComputeSearchKeys')
    const configurations = [...configurations1, ...configurations2];
    logger.noop('configurations', configurations)
    const roles = proposition.RoleNames()
    logger.noop({ roles }, ComputeSearchKeys)
    var result = []
    for (const configuration of configurations) {
        const quantifiedRoles = ExtractRolesFromIndices(roles, configuration)
        logger.noop({ quantifiedRoles }, ComputeSearchKeys)
        const quantified = ConvertToQuantified(proposition, quantifiedRoles)
        logger.noop({ quantified }, ComputeSearchKeys)
        const searchString = quantified.SearchString()
        logger.noop({ searchString }, ComputeSearchKeys)
        result.push(searchString)
    }
    return result
}

async function ComputeBacklinks(storage, proposition) {
    assert.isType(proposition, Proposition)
    assert.isTrue(proposition.IsFact())
    assert.isType(storage, Storage)
    const searchKeys = ComputeSearchKeys(proposition)
    logger.noop({ searchKeys }, ComputeBacklinks)
    var buffer = []
    for (const searchKey of searchKeys) {
        logger.noop({searchKey}, ComputeBacklinks)
        const implications = await storage.FindPremises(searchKey)
        for (const implication of implications) {
            logger.noop({ implication }, ComputeBacklinks)

            const extractedMapping = ExtractPremiseRoleMap(proposition, implication.roleMap)
            logger.noop({ extractedMapping }, ComputeBacklinks)

            const quantifiedPremise = implication.premise
            logger.noop({ quantifiedPremise }, ComputeBacklinks)

            const extractedProposition = ConvertToProposition(quantifiedPremise, extractedMapping)
            logger.noop({ extractedProposition }, ComputeBacklinks)
            buffer.push(new BackLink(implication, extractedProposition));
        }
    }
    return buffer
}

async function ComputeMatchingPremises(storage, premise) {
    assert.isType(storage, Storage)
    assert.isType(premise, Proposition)
    assert.isTrue(premise.isQuantified())
    return []
}

module.exports = { ComputeSearchKeys, ComputeBacklinks, ComputeMatchingPremises }