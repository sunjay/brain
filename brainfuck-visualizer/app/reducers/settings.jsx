const {createReducer} = require('./reducer');

const Settings = require('../models/settings');

module.exports = createReducer(new Settings(), {
});
