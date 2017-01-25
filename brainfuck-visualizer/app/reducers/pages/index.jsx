const {combineReducers} = require('redux');

const history = require('../history');
const output = require('../output');
const settings = require('../settings');

module.exports = combineReducers({
  history,
  output,
  settings,
});
