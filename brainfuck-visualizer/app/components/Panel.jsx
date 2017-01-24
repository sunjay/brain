const React = require('react');
const classNames = require('classnames');

const {panel} = require('../../scss/components/panel.scss');

const Panel = ({children, className}) => (
  <div className={classNames(panel, className)}>
    {children}
  </div>
);

Panel.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Panel;
