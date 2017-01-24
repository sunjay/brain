const React = require('react');

require('../../scss/index.scss');

const {app} = require('../../scss/components/app.scss');

const App = React.createClass({
  propTypes: {
    children: React.PropTypes.node,
  },

  render() {
    const {children} = this.props;

    return (
      <div className={app}>
        {children}
      </div>
    );
  },
});


module.exports = App;
