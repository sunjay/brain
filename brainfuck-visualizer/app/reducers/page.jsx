const {LOCATION_CHANGE} = require('react-router-redux');

// Private property for keeping the pathname
const PATHNAME = Symbol('pathname');

/**
 * Reducer for managing page-specific/route-specific data
 *
 * Automatically resets the state on every LOCATION_CHANGE
 * action as provided by react-router-redux
 * Matches the current location pathname provided by react-router-redux
 * to a reducer given by the pageReducers array
 *
 * @param {Object[]} pageReducers - Array of {pattern, reducer} objects
 *   representing which reducer to use for URLs that match the given RegExp
 * @returns {Function} - Returns a reducer function
 */
const createPageReducer = (pageReducers = []) => {
  // explicitly putting undefined here so no one defines a default
  return (state = undefined, action) => {
    let pathname;
    if (action.type === LOCATION_CHANGE) {
      state = undefined;
      pathname = action.payload.pathname;
    }
    else {
      pathname = (state || {})[PATHNAME];
    }

    state = pageReducers.reduce((state, {pattern, reducer}) => {
      if (pathname && pattern.test(pathname)) {
        return reducer(state, action);
      }
      return state;
    }, state);

    state = state || {};
    state[PATHNAME] = pathname;
    return state;
  };
};

module.exports = createPageReducer;
