const {createReducer} = require('./reducer');

const {
  ACTION_APPEND_OUTPUT,
  ACTION_APPEND_ERROR,
} = require('../actions/InterpreterActions');

// The string output of the interpreter
module.exports = createReducer([], {
  [ACTION_APPEND_OUTPUT](state, {output}) {
    return [...state, {output}];
  },

  [ACTION_APPEND_ERROR](state, {error}) {
    return [...state, {error}];
  },
});
