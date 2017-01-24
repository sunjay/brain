const {connect} = require('react-redux');

const SettingsPanel = require('../components/SettingsPanel');

const mapStateToProps = ({page: {settings}}) => ({
  ...settings.toJSON(),
});

const mapDispatchToProps = (dispatch) => ({
});

module.exports = connect(
  mapStateToProps,
  mapDispatchToProps
)(SettingsPanel);
