const {createRecord} = require('./model');

const SettingsRecord = createRecord({
  DEFAULT_BRAINFUCK_COMMAND: 'brainfuck --debug --delay 10',
}, (constants) => ({
  command: constants.DEFAULT_BRAINFUCK_COMMAND,
  file: undefined,
  source: undefined,
  setup: false,
}));

class Settings extends SettingsRecord {
}

module.exports = Settings;
