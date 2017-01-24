const React = require('react');
const classNames = require('classnames');

const {page} = require('../../scss/components/page.scss');

const Page = ({children, className}) => (
  <div className={classNames(page, className)}>
    {children}
  </div>
);

Page.propTypes = {
  children: React.PropTypes.any,
  className: React.PropTypes.string,
};

module.exports = Page;
