const logger = require("../logger")
const { ConstantArgument, VariableArgument, Proposition, FilledRole, Implication, Entity, RoleMap } = require("./predicate")
const { CreateRedisClient } = require("./redis")
const { CreateStorage } = require("./storage")

function implication(premise, conclusion, roleMap) {
    return new Implication(premise, conclusion, roleMap)
}

function predicate(roles) {
    return new Proposition(roles)
}
function role(role_name, argument) {
    logger.noop({ role_name, argument }, role)
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
    const redis = await CreateRedisClient()
    await redis.DropAllDBs()
    const storage = await CreateStorage(redis)

    const TOTAL_MEMBERS_EACH_CLASS = 32
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

    let exciting = constant('verb', "exciting");
    let lonely = constant('verb', "lonely");
    let like = constant('verb', "like");
    let date = constant('verb', "date");

    var independentFactMap = {} // store this locally for efficiency, because the MongoDB is slow rn
    // for each jack: coinflip to determine if "lonely(jill)"
    for (const jackEntity of jacks) {
        let jack = constant(jackEntity.domain, jackEntity.name);
        const jackLonely = predicate([subject(jack), relation(lonely)]);
        const pJackLonely = cointoss()
        logger.noop({ jackLonely, pJackLonely }, main)
        await storage.StoreProposition(jackLonely, pJackLonely)
        independentFactMap[jackLonely.ToString()] = pJackLonely
    }
    // for each jill: coinflip to determine if "exciting(jill)"
    for (const jillEntity of jills) {
        let jill = constant(jillEntity.domain, jillEntity.name);
        const jillExciting = predicate([subject(jill), relation(exciting)]);
        const pJillExciting = cointoss()
        logger.noop({ jillExciting, pJillExciting }, main)
        await storage.StoreProposition(jillExciting, pJillExciting)
        independentFactMap[jillExciting.ToString()] = pJillExciting
    }

    for (const jackEntity of jacks) {
        for (const jillEntity of jills) {
            let jill = constant(jillEntity.domain, jillEntity.name);
            let jack = constant(jackEntity.domain, jackEntity.name);
            // for each [jill, jack]: coinflip to determine if "likes(jill, jack)"
            {
                const jillLikesJack = predicate([subject(jill), relation(like), object(jack)]);
                const pJillLikesJack = cointoss()
                logger.noop({ jillLikesJack, pJillLikesJack }, main)
                await storage.StoreProposition(jillLikesJack, pJillLikesJack)
                independentFactMap[jillLikesJack.ToString()] = pJillLikesJack
            }
            // for each [jill, jack]: deterministically say that "likes(jack, jill)" iff "lonely(jack) or exciting(jill)"
            {
                const jackLonely = predicate([subject(jack), relation(lonely)]);
                const pJackLonely = independentFactMap[jackLonely.ToString()]
                logger.noop({ jackLonely, pJackLonely }, main)
                const jillExciting = predicate([subject(jill), relation(exciting)]);
                const pJillExciting = independentFactMap[jillExciting.ToString()]
                logger.noop({ jillExciting, pJillExciting }, main)
                function numeric_or(a, b) {
                    return Math.min(a + b, 1)
                }
                const jackLikesJill = predicate([subject(jack), relation(like), object(jill)]);
                const pJackLikesJill = numeric_or(pJackLonely, pJillExciting);
                logger.noop({ pJackLonely, pJillExciting, pJackLikesJill }, main)
                await storage.StoreProposition(jackLikesJill, pJackLikesJill)
                logger.noop({ jackLikesJill, pJackLikesJill }, main)
                independentFactMap[jackLikesJill.ToString()] = pJackLikesJill
            }
            // for each [jill, jack]: deterministically say that "dates(jack, jill)" iff "likes(jack,jill) and likes(jill, jack)"
            {
                const jackLikesJill = predicate([subject(jack), relation(like), object(jill)]);
                const pJackLikesJill = independentFactMap[jackLikesJill.ToString()]
                const jillLikesJack = predicate([subject(jill), relation(like), object(jack)]);
                const pJillLikesJack = independentFactMap[jillLikesJack.ToString()]
                function numeric_and(a, b) {
                    return a * b
                }
                const jackDatesJill = predicate([subject(jack), relation(date), object(jill)]);
                const pJackDatesJill = numeric_and(pJackLikesJill, pJillLikesJack)
                logger.noop({ jackLikesJill, jillLikesJack, pJackLikesJill, pJillLikesJack, pJackDatesJill }, main)
                await storage.StoreProposition(jackDatesJill, pJackDatesJill)
            }
        }
    }

    let xjack = variable("jack");
    let xjill = variable("jill");
    const implications = [
        // if jack is lonely, he will date any jill
        implication(
            predicate([subject(xjack), relation(lonely)]),
            predicate([subject(xjack), relation(like), object(xjill)]),
            new RoleMap({ "subject": "subject" })
        ),
        // if jill is exciting, any jack will date her
        implication(
            predicate([subject(xjill), relation(exciting)]),
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
    logger.noop({ num_implications: implications.length }, main)
    for (const implication of implications) {
        logger.noop({implication}, main)
        await storage.StoreImplication(implication)

    }

    await redis.Disconnect()
}

main()