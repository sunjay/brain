const React = require('react');
const classNames = require('classnames');

const {
  tapeContainer,
  tape,
  tapeCell,
  tapeCurrent,
  tapeCellIndex,
  tapeCellValue,
} = require('../../scss/components/tape.scss');

const Tape = ({currentPointer, memory}) => (
  <div className={tapeContainer}>
    <div className={tape}>
      {(memory || []).map((cell, index) => (
        <div key={index} className={tapeCell}>
          <div className={tapeCellIndex}>
            {index}
          </div>
          <div className={tapeCellIndex}>
            <code>'{
              String.fromCharCode(cell)
                .replace('\n', '\\n')
                .replace('\t', '\\t')
                .replace('\b', '\\b')
                .replace('\r', '\\r')
                .replace('\r', '\\r')
                .replace('\v', '\\v')
            }'</code>
          </div>
          <div className={classNames({
            [tapeCellValue]: true,
            [tapeCurrent]: index === currentPointer,
          })}>
            {cell}
          </div>
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
