const {connect} = require('react-redux');

const SettingsPanel = require('../components/SettingsPanel');

const {start} = require('../actions/InterpreterActions');

const mapStateToProps = ({page: {settings}}) => ({
  ...settings.toJSON(),
});

const mapDispatchToProps = (dispatch) => ({
  onSubmit(command, file) {
    dispatch(start(command, file));
  },
});

module.exports = connect(
  mapStateToProps,
  mapDispatchToProps
)(SettingsPanel);
