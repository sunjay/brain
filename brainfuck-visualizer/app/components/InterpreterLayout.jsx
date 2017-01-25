const React = require('react');

const Tape = require('../containers/Tape');
const Output = require('../containers/Output');

const InterpreterLayout = ({setup}) => setup ? (
  <div>
    <Tape />
    <Output />
  </div>
) : null;

InterpreterLayout.propTypes = {
  command: React.PropTypes.string,
  file: React.PropTypes.string,
};

module.exports = InterpreterLayout;
