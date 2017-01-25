const React = require('react');

const {
  outputContainer,
  outputError,
} = require('../../scss/components/output.scss');

const Output = ({output}) => (
  <pre className={outputContainer}>
    {output.map(({output, error}, index) => output ? output : (
      <div key={index} className={outputError}>
        {error}
      </div>
    ))}
  </pre>
);

Output.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Output;
