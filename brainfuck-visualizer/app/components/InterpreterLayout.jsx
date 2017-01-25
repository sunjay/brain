const React = require('react');

const Tape = require('../containers/Tape');

const InterpreterLayout = ({setup}) => setup ? (
  <div>
    <Tape />
  </div>
) : null;

InterpreterLayout.propTypes = {
  command: React.PropTypes.string,
  file: React.PropTypes.string,
};

module.exports = InterpreterLayout;
