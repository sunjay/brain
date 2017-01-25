const Actions = require('./Actions');

export const ACTION_START = Actions.register('interpreter-start');
export const ACTION_APPEND_OUTPUT = Actions.register('interpreter-append-output');
export const ACTION_APPEND_ERROR = Actions.register('interpreter-append-error');
export const ACTION_ADD_HISTORY = Actions.register('interpreter-add-history');
export const ACTION_SET_SOURCE = Actions.register('interpreter-set-source');

export const start = Actions.registerActionCreator(
  ACTION_START,
  [
    'command',
    'file',
  ]
);

export const appendOutput = Actions.registerActionCreator(
  ACTION_APPEND_OUTPUT,
  [
    'output',
  ]
);

export const appendError = Actions.registerActionCreator(
  ACTION_APPEND_ERROR,
  [
    'error',
  ]
);

export const addHistory = Actions.registerActionCreator(
  ACTION_ADD_HISTORY,
  [
    'record',
  ]
);

export const setSource = Actions.registerActionCreator(
  ACTION_SET_SOURCE,
  [
    'source',
  ]
);
