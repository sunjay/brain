const {combineReducers} = require('redux');

const interpreterState = require('../interpreterState');
const settings = require('../settings');

module.exports = combineReducers({
  interpreterState,
  settings,
});
