const mongoose = require("mongoose")

const logger = require("../logger")
const { ConstantArgument, VariableArgument, Proposition, FilledRole, Implication, Entity, RoleMap } = require("./predicate")
const { CreateStorage, ConnectDB } = require("./storage")

function implication(premise, conclusion, roleMap) {
    return new Implication(premise, conclusion, roleMap)
}

function predicate(roles) {
    return new Proposition(roles)
}
function role(role_name, argument) {
    return new FilledRole(role_name, argument)
}
function variable(domain) {
    return new VariableArgument(domain)
}
function constant(domain, entity_id) {
    return new ConstantArgument(domain, entity_id)
}
function subject(argument) {
    return role("subject", argument)
}
function object(argument) {
    return role("object", argument)
}

function relation(argument) {
    return role("relation", argument)
}


async function main() {
    await ConnectDB();
    const storage = await CreateStorage("testdb1")
    await storage.DropAllDBs()

    const TOTAL_MEMBERS_EACH_CLASS = 16
    const domains = ['jack', 'jill']
    for (const domain of domains) {
        for (var i = 0; i < TOTAL_MEMBERS_EACH_CLASS; i++) {
            const name = `${domain}${i}`
            const entity = new Entity(domain, name)
            logger.noop({ entity }, main)
            await storage.StoreEntity(entity)
        }
    }

    const jacks = await storage.GetEntitiesInDomain('jack')
    const jills = await storage.GetEntitiesInDomain('jill')
    logger.noop({ num_jacks: jacks.length, num_jills: jills.length }, main)

    function cointoss() {
        return Math.random() < 0.5 ? 1.0 : 0.0;
    }

    let exciting = constant('predicate', "exciting");
    let lonely = constant('predicate', "lonely");
    let like = constant('relation', "like");
    let date = constant('relation', "date");
    let is = constant('relation', "is");
    
    var independentFactMap = {} // store this locally for efficiency, because the MongoDB is slow rn
    for (const jackEntity of jacks) {
        let jack = constant(jackEntity.domain, jackEntity.name);
        const jackLonely = predicate([subject(jack), relation(lonely)]);
        const pJackLonely = cointoss()
        logger.dump({jackLonely, pJackLonely}, main)
        await storage.StoreProposition(jackLonely, pJackLonely)
        independentFactMap[jackLonely.ToString()] = pJackLonely
    }
    for (const jillEntity of jills) {
        let jill = constant(jillEntity.domain, jillEntity.name);
        const jillExciting = predicate([subject(jill), relation(exciting)]);
        const pJillExciting = cointoss()
        logger.dump({jillExciting, pJillExciting}, main)
        await storage.StoreProposition(jillExciting, pJillExciting)
        independentFactMap[jillExciting.ToString()] = pJillExciting
    }
    for (const jackEntity of jacks) {
        for (const jillEntity of jills) {
            const jillLikesJack = predicate([subject(jillEntity), relation(like), object(jackEntity)]);
            const pJillLikesJack = cointoss()
            facts.push([jillLikesJack, pJillLikesJack])
            logger.dump({jillLikesJack, pJillLikesJack}, main)
            await storage.StoreProposition(jillLikesJack, pJillLikesJack)
            independentFactMap[jillLikesJack.ToString()] = pJillLikesJack
        }
    }

    process.exit()
    var jillExcitingMap = {}
    for (const jillEntity of jills) {

    }
    for (const jackEntity of jacks) {
        var facts = []
        for (const jillEntity of jills) {
            logger.noop({ jackEntity, jillEntity }, main)
            let jill = constant(jillEntity.domain, jillEntity.name);
            const jillExciting = cointoss()
            const jackLonely = cointoss()
            const jillLikesJack = cointoss()
            const jackLikesJill = jillExciting || jackLonely;
            const jackDatesJill = jillLikesJack && jackLikesJill;
            {

            }
            {
                const proposition = predicate([subject(jack), relation(is), object(lonely)]);
                facts.push([proposition, jackLonely])
            }
            {
                const proposition = predicate([subject(jill), relation(like), object(jack)]);
                facts.push([proposition, jillLikesJack])
            }
            {
                const proposition = predicate([subject(jack), relation(like), object(jill)]);
                facts.push([proposition, jackLikesJill])
            }
            {
                const proposition = predicate([subject(jack), relation(date), object(jill)]);
                facts.push([proposition, jackDatesJill])
            }
        }
        for (const [fact, value] of facts) {
            logger.noop({ fact, value }, main)
            await storage.StoreProposition(fact, value)
        }
    }

    let xjack = variable("jack");
    let xjill = variable("jill");
    const implications = [
        // if jack is lonely, he will date any jill
        implication(
            predicate([subject(xjack), relation(is), object(lonely)]),
            predicate([subject(xjack), relation(like), object(xjill)]),
            new RoleMap({ "subject": "subject" })
        ),
        // if jill is exciting, any jack will date her
        implication(
            predicate([subject(xjill), relation(is), object(exciting)]),
            predicate([subject(xjack), relation(like), object(xjill)]),
            new RoleMap({ "object": "subject" })
        ),
        // if jill likes jack, then jack dates jill
        implication(
            predicate([subject(xjill), relation(like), object(xjack)]),
            predicate([subject(xjack), relation(date), object(xjill)]),
            new RoleMap({ "subject": "object", "object": "subject" })
        ),
        // if jack likes jill, then jack dates jill
        implication(
            predicate([subject(xjack), relation(like), object(xjill)]),
            predicate([subject(xjack), relation(date), object(xjill)]),
            new RoleMap({ "subject": "subject", "object": "object" })
        ),
    ]
    logger.noop({ implications }, main)
    for (const implication of implications) {
        await storage.StoreImplication(implication)

    }
    await mongoose.disconnect();
}

main()