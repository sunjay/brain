const {connect} = require('react-redux');

const InterpreterLayout = require('../components/InterpreterLayout');

const mapStateToProps = ({page: {settings: {setup}}}) => ({
  setup,
});

const mapDispatchToProps = (dispatch) => ({
});

module.exports = connect(
  mapStateToProps,
  mapDispatchToProps
)(InterpreterLayout);
