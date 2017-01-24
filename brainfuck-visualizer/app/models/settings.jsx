const {createRecord} = require('./model');

const SettingsRecord = createRecord({
  DEFAULT_BRAINFUCK_EXECUTABLE: 'brainfuck',
}, (constants) => ({
  command: constants.DEFAULT_BRAINFUCK_EXECUTABLE,
  file: undefined,
}));

class Settings extends SettingsRecord {
}

module.exports = Settings;
