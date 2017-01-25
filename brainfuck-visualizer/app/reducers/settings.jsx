const {createReducer} = require('./reducer');

const {
  ACTION_START,
  ACTION_SET_SOURCE,
} = require('../actions/InterpreterActions');

const Settings = require('../models/settings');

module.exports = createReducer(new Settings(), {
  [ACTION_START](state, {command, file}) {
    return state.merge({command, file, setup: true});
  },

  [ACTION_SET_SOURCE](state, {source}) {
    return state.set('source', source);
  },
});
