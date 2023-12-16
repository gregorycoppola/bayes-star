const assert = require("../assert")
const logger = require("../logger")

// These are the only domains allowed. This can change.
const domainSet = new Set(['jack', 'jill', 'verb'])
// These are the only types of arguments. This can change, but two might be.
const typeSet = new Set(['constant', 'variable'])
class FirstOrderArgument {
    constructor(type, domain) {
        this.type = type;
        this.domain = domain;
        logger.noop({me: this}, FirstOrderArgument)
        assert.isTrue(typeSet.has(type), FirstOrderArgument)
        assert.isTrue(domainSet.has(domain), FirstOrderArgument)
    }

    IsVariable() {
        return this.type == 'variable'
    }
    static FromTuple(tuple) {
        switch (tuple.type) {
            case "constant":
                return ConstantArgument.FromTuple(tuple)
            case "variable":
                return VariableArgument.FromTuple(tuple)
            default:
                throw new Error("not found!")
        }
    }
}

class ConstantArgument extends FirstOrderArgument {
    constructor(domain, entity_id) {
        super("constant", domain);
        assert.isType(entity_id, "string")
        assert.isType(domain, "string")
        this.entity_id = entity_id;
    }
    SearchString() {
        const cstring = `${this.entity_id}`
        logger.noop({cstring, obj: this}, this.searchString)
        return cstring
    }
    ConvertToQuantified() {
        return new VariableArgument(this.domain)
    }

    static FromTuple(tuple) {
        return new ConstantArgument(tuple.domain, tuple.entity_id)
    }
}

const BOUND_VARIABLE = '?'
class VariableArgument extends FirstOrderArgument {
    constructor(domain) {
        super("variable", domain);
        assert.isType(domain, "string")
    }
    SearchString() {
        return `${BOUND_VARIABLE}${this.domain}`
    }

    static FromTuple(tuple) {
        return new VariableArgument(tuple.domain)
    }
}

class FilledRole {
    constructor(role_name, argument) {
        assert.isType(role_name, "string")
        assert.isType(argument, FirstOrderArgument)
        this.role_name = role_name
        this.argument = argument
    }
    SearchString() {
        const roleString =  `${this.role_name}=${this.argument.SearchString()}`
        logger.noop({roleString}, this.searchString)
        return roleString
    }
    ConvertToQuantified() {
        return new FilledRole(this.role_name, this.argument.ConvertToQuantified())
    }
    DoSubstitution(value) {
        return new FilledRole(this.role_name, value)
    }

    static FromTuple(tuple) {
        const role_name = tuple.role_name
        const argument = FirstOrderArgument.FromTuple(tuple.argument)
        return new FilledRole(role_name, argument)
    }
}

class Proposition {
    constructor(roles) {
        logger.noop({roles}, Proposition)
        assert.arrayOf(roles, FilledRole)
        roles.sort((a, b) => {
            if (a.role_name < b.role_name) {
                return -1;
            }
            if (a.role_name > b.role_name) {
                return 1;
            }
            return 0;
        });
        logger.noop({roles})
        this.roles = roles
    }
    RoleNames() {
        var result = []
        for (const role of this.roles) {
            result.push(role.role_name)
        }
        return result
    }
    SearchString() {
        var result = '['
        var started = false
        for (const column of this.roles) {
            if (started) {
                result += ', '
            }
            result += column.SearchString()
            started = true
        }
        result += ']'
        return result
    }
    IsFact() {
        this.roles.forEach((column) => {

            if (column.argument.type == 'variable') {
                return false
            } else {
                // console.log('not quantified', {column})
            }
        })
        return true
    }
    ToString() {
        return JSON.stringify(this.roles)
    }

    static FromTuple(tuple) {
        logger.noop({confirm: true, tuple}, this.FromTuple)
        var r = []
        for (const roleTuple of tuple['roles']) {
            r.push(FilledRole.FromTuple(roleTuple))
        }
        return new Proposition(r)
    }
    static equal(a, b) {
        return a.ToString() == b.ToString()
    }
}

function ToStringObjectCanonical(obj) {
    const sortedObj = {};
    Object.keys(obj).sort().forEach(function(key) {
        sortedObj[key] = obj[key];
    });
    return JSON.stringify(sortedObj);
}
class Implication {
    constructor(premise, conclusion, roleMap) {
        logger.noop({premise, conclusion, roleMap}, Implication)
        assert.isType(premise, Proposition)
        assert.isType(conclusion, Proposition)
        assert.isType(roleMap, RoleMap)
        this.premise = premise
        this.conclusion = conclusion
        this.roleMap = roleMap
        // TODO: Check variable mapping.
    }

    SearchString() {
        return this.conclusion.SearchString()
    }
    UniqueKey() {
        return `${this.premise.SearchString()}->${this.conclusion.SearchString()}${this.MappingString()}`
    }
    FeatureString() {
        return `${this.premise.SearchString()}${this.MappingString()}`
    }
    MappingString() {
        return this.roleMap.ToString()
    }
    ToString() {
        logger.noop({ToString: this}, this.ToString)
        var r = {}
        Object.entries(this).forEach(([key, value]) => {
            assert.isTrue(value.ToString())
            r[key] = value.ToString()
        })
        return JSON.stringify(r)
    }
    static FromTuple(tuple) {
        logger.noop({tuple}, this.FromTuple)
        // const {premise, conclusion, roleMap} = tuple
        const premise = tuple['premise']
        const conclusion = tuple['conclusion']
        const roleMap = tuple['roleMap']
        logger.noop({premise, conclusion, roleMap}, this.FromTuple)
        return new Implication(Proposition.FromTuple(premise), Proposition.FromTuple(premise), RoleMap.FromTuple(roleMap))
    }
}

class Entity {
    constructor(domain, name) {
        this.domain = domain
        this.name = name
    }
}

class RoleMap {
    constructor(roleMap) {
        this.roleMap = roleMap
    }
    Get(role_name) {
        const rval = this.roleMap[role_name]
        logger.noop({role_name, roleMap:this.roleMap, rval}, this.Get)
        return rval
    }
    ToString() {
        return ToStringObjectCanonical(this.roleMap)
    }

    static FromTuple(tuple) {
        return new RoleMap(tuple['roleMap'])
    }
}

class BackLink {
    constructor(implication, proposition) {
        logger.noop({implication, proposition}, BackLink)
        assert.isType(implication, Implication)
        assert.isType(proposition, Proposition)
        this.implication = implication
        this.proposition = proposition
    }
}

module.exports = { ConstantArgument, VariableArgument, Proposition, Implication, FilledRole, Entity, RoleMap, BackLink }