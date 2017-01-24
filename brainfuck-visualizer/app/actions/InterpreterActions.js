const Actions = require('./Actions');

export const ACTION_START = Actions.register('interpreter-start');

exports.start = Actions.registerActionCreator(
  exports.ACTION_START,
  [
    'command',
    'file',
  ]
);
