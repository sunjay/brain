const {combineReducers} = require('redux');
const {routerReducer: routing} = require('react-router-redux');

const routes = require('../routes');
const createPageReducer = require('./page');

const index = require('./pages/index');

const page = createPageReducer([
  {pattern: routes.index.pattern, reducer: index},
]);

// Reducers placed here are the top-level of the state object
// Each of these reducers can be composed of zero or more other reducers
// in order to make whatever state shape you desire
// Reducers placed here are common to every page
// Page-specific/route-specific state should be placed in
// a page-specific reducer defined in the createPageReducer call
const app = combineReducers({
  routing,
  page,
});

module.exports = app;
