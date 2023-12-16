const { Proposition, FirstOrderArgument, RoleMap } = require("./predicate");
const logger = require("../logger")
const assert = require("../assert")

function ConvertToQuantified(proposition, roles) {
    const roleSet = new Set(roles)
    var result = []
    for (let i = 0; i < proposition.roles.length; i++) {
        const crole = proposition.roles[i]
        const role_name = crole.role_name
        if (roleSet.has(role_name)) {
            const converted = crole.ConvertToQuantified()
            logger.noop({has:true, converted}, ConvertToQuantified)
            result.push(converted)
        } else {
            logger.noop({has:false, roles}, ConvertToQuantified)
            result.push(crole)
        }
    }
    logger.noop({result}, ConvertToQuantified)
    return new Proposition(result)
}

function ConvertToProposition(predicate, roleMap) {
    assert.mapOf(roleMap, "string", FirstOrderArgument)
    logger.noop({predicate, roleMap}, ConvertToProposition)
    var result = []
    for (let i = 0; i < predicate.roles.length; i++) {
        const role = predicate.roles[i]
        const IsVariable = role.argument.IsVariable()
        logger.noop({role, IsVariable}, ConvertToProposition)
        if (IsVariable) {
            const substitute = roleMap.Get(role.role_name)
            logger.noop({roleMap, role, substitute}, ConvertToProposition)
            assert.isTrue(substitute)
            const newRole = role.DoSubstitution(substitute)
            logger.noop({newRole}, ConvertToProposition)
            result.push(newRole)
        } else {
            result.push(role)
        }
    }
    return new Proposition(result)
}

function ExtractPremiseRoleMap(proposition, roleMap) {
    assert.isType(proposition, Proposition)
    assert.isType(roleMap, RoleMap)
    logger.dump({proposition, roleMap}, ExtractPremiseRoleMap)
    var result = {}
    for (let i = 0; i < proposition.roles.length; i++) {
        const crole = proposition.roles[i]
        const role_name = crole.role_name
        const premise_role_name = roleMap.Get(role_name)

        logger.dump({crole, role_name, roleMap, premise_role_name}, ExtractPremiseRoleMap)
        if (premise_role_name) {
            result[premise_role_name] = crole.argument
        }
    }
    logger.noop({result}, ExtractPremiseRoleMap)
    const rval = new RoleMap(result)
    assert.isType(rval, RoleMap)
    return rval
}

module.exports = { ConvertToQuantified, ConvertToProposition, ExtractPremiseRoleMap}
