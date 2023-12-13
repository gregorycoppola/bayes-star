const assert = require("./assert")

function dump(jsonObject, identifier) {
    assert.isTrue(identifier)
    console.log(identifier, JSON.stringify(jsonObject, null, 2)); // 4 spaces for indentation
}

function note(message, identifier) {
    assert.isTrue(identifier)
    console.log(identifier, message)
}

function todo(message, identifier) {}

function trace() {
    const stack = new Error().stack;
    console.log(stack);
}

function noop() {
    // noop
}

module.exports = {dump, note, todo, trace, noop}