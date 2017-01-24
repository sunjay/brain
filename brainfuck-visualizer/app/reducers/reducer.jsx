/**
 * Automatically dispatches action types to an appropriate reducer function.
 * Avoids huge switch statements.
 * From: http://redux.js.org/docs/recipes/ReducingBoilerplate.html#generating-reducers
 * @param {*} initialState - the initial state that the reducer should return initially
 * @param {Object} handlers - Mapping of action type to reducer function
 * @returns {Function} Returns a reducer that calls the given handler function for each
 *    action type
 */
export function createReducer(initialState = null, handlers = {}) {
  return function reducer(state = initialState, action) {
    if (handlers.hasOwnProperty(action.type)) {
      return handlers[action.type](state, action);
    }
    return state;
  };
}
