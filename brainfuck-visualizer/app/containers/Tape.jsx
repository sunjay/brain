const {connect} = require('react-redux');

const Tape = require('../components/Tape');

const mapStateToProps = ({page: {interpreterState}}) => ({
});

const mapDispatchToProps = (dispatch) => ({
});

module.exports = connect(
  mapStateToProps,
  mapDispatchToProps
)(Tape);
