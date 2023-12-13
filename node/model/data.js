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
                const pJackDatesJill = numeric_and(pJackLikesJill, pJillLikesJack)
                logger.note({ pJackLikesJill, pJillLikesJack, pJackDatesJill }, main)

                // logger.note({ pJackLonely, pJillExciting, pJackLikesJill }, main)
                // await storage.StoreProposition(jackLikesJill, pJackLikesJill)
                // logger.note({ jackLikesJill, pJackLikesJill }, main)
                // independentFactMap[jackLikesJill.ToString()] = pJackLikesJill
            }
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