const React = require('react');

const Tape = ({children, className}) => (
  <div className={className}>
    <h1>Tape!</h1>
    {children}
  </div>
);

Tape.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Tape;
