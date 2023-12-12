
const mongoose = require("mongoose")

const factSchema = new mongoose.Schema({
    searchString: { type: String, required: true, unique: true },
    record: { type: String, required: true },
    probability: { type: Number, required: true },
});

const PropositionRecord = mongoose.model('Proposition', factSchema);

const implicationSchema = new mongoose.Schema({
    UniqueKey: { type: String, required: true, unique: true },
    searchString: { type: String, required: true },
    featureString: { type: String, required: true },
    premiseRecord: { type: String, required: true },
    conclusionRecord: { type: String, required: true },
    mappingRecord: { type: String, required: true },
});

const ImplicationRecord = mongoose.model('Implication', implicationSchema);

const entitySchema = new mongoose.Schema({
    name: { type: String, required: true, unique: true },
    domain: { type: String, required: true },
})
const EntityRecord = mongoose.model('Entity', entitySchema);

module.exports = {PropositionRecord, ImplicationRecord, EntityRecord}