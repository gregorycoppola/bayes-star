function isType(instance, type) {
    if (type === 'array') { // Specific check for arrays
        if (!Array.isArray(instance)) {
            throw new Error(`Expected an array, but got ${typeof instance}`);
        }
        return;
    }

    if (type === 'function') { // Specific check for functions
        if (typeof instance !== 'function') {
            throw new Error(`Expected a function, but got ${typeof instance}`);
        }
        return;
    }

    if (instance === null || instance === undefined) { // Handle null and undefined
        throw new Error(`Expected ${type}, but got ${instance}`);
    }

    if (typeof instance !== 'object' && typeof type === 'string') { // Primitive type check
        if (typeof instance !== type || (type === 'number' && isNaN(instance))) {
            throw new Error(`Expected type of ${type}, but got ${typeof instance}`);
        }
    } else if (!(instance instanceof type)) { // Object type check
        throw new Error(`Expected instance of ${type.name}, but got ${instance.constructor ? instance.constructor.name : typeof instance}`);
    }
}

function arrayOf(instance, type) {
    if (!Array.isArray(instance)) {
        throw new Error(`Expected an array, but got ${typeof instance}`);
    }

    instance.forEach((element, index) => {
        try {
            isType(element, type);
        } catch (error) {
            throw new Error(`Error in array at index ${index}: ${error.message}`);
        }
    });
}

function setOf(instance, type) {
    if (!(instance instanceof Set)) {
        throw new Error(`Expected a Set, but got ${typeof instance}`);
    }

    instance.forEach((element, index) => {
        try {
            isType(element, type);
        } catch (error) {
            throw new Error(`Error in Set at element with value ${element}: ${error.message}`);
        }
    });
}

function mapOf(instance, keyType) {
    // Check if instance is a plain object
    if (typeof instance !== 'object' || instance === null || Array.isArray(instance)) {
        throw new Error(`Expected a JSON object, but got ${typeof instance}`);
    }

    // Iterate over the object's keys and values
    Object.entries(instance).forEach(([key, value]) => {
        try {
            isType(key, keyType);
        } catch (error) {
            throw new Error(`Error in object at key '${key}': ${error.message}`);
        }

        // // Check the type of the value
        // try {
        //     isType(value, valueType);
        // } catch (error) {
        //     throw new Error(`Error in object at key '${key}' with value '${value}': ${error.message}`);
        // }
    });
}

function isTrue(expression) {
    if (!expression) {
        throw new Error(`This was supposed to be true ${expression}`);
    }
}

function isEqual(a, b) {
    if (a!= b) {
        throw new Error(`these are supposed to be equal ${a} vs. ${b}`);
    }
}

module.exports = { isType, arrayOf, isTrue, isEqual, setOf, mapOf }