const {createRecord} = require('./model');

const InterpreterStateRecord = createRecord({
}, (constants) => ({
  lastInstructionIndex: undefined,
  lastInstruction: undefined,
  currentPointer: undefined,
  memory: undefined,
}));

class InterpreterState extends InterpreterStateRecord {
}

module.exports = InterpreterState;
