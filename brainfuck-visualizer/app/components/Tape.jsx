const React = require('react');

const Tape = ({children, className}) => (
  <div className={className}>
    {children}
  </div>
);

Tape.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Tape;
