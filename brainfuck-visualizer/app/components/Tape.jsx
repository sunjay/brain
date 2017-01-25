const React = require('react');
const classNames = require('classnames');

const {
  tapeContainer,
  tape,
  tapeCell,
  tapeCurrent,
} = require('../../scss/components/tape.scss');

const Tape = ({currentPointer, memory}) => (
  <div className={tapeContainer}>
    <div className={tape}>
      {(memory || []).map((cell, index) => (
        <div key={index} className={classNames({
          [tapeCell]: true,
          [tapeCurrent]: index === currentPointer,
        })}>
          {cell}
        </div>
      ))}
    </div>
  </div>
);

Tape.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Tape;
