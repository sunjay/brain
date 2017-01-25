require('babel-polyfill');

const React = require('react');
const ReactDOM = require('react-dom');
const {Provider} = require('react-redux');
const {compose, createStore, applyMiddleware} = require('redux');
const {hashHistory} = require('react-router');
const {routerMiddleware, syncHistoryWithStore} = require('react-router-redux');
const createLogger = require('redux-logger');
const thunk = require('redux-thunk').default;

const createRouter = require('./router');
const appReducer = require('./reducers/app');

const Interpreter = require('./services/interpreter');
const {ACTION_ADD_HISTORY} = require('./actions/InterpreterActions');

const interpreter = new Interpreter();

const logger = createLogger({
  predicate(getState, {type}) {
    return type !== ACTION_ADD_HISTORY;
  },
});
const store = createStore(
  appReducer,
  compose(
    applyMiddleware(
      routerMiddleware(hashHistory),
      thunk,
      interpreter.middleware(),
      // The logger MUST be last (other than DevTools)
      logger
    ),
    window.devToolsExtension ? window.devToolsExtension() : f => f
  )
);

const history = syncHistoryWithStore(hashHistory, store);

ReactDOM.render(
  <Provider store={store}>
    {createRouter(history, store)}
  </Provider>,
  document.getElementById('app-container')
);

interpreter.start(store);
