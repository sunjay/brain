const Actions = require('./Actions');

export const ACTION_RELOAD = Actions.register('exec-reload');

exports.reload = Actions.registerActionCreator(
  exports.ACTION_RELOAD
);
