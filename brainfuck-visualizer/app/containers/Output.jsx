const {connect} = require('react-redux');

const Output = require('../components/Output');

const mapStateToProps = ({page: {output}}) => ({
  output,
});

const mapDispatchToProps = (dispatch) => ({
});

module.exports = connect(
  mapStateToProps,
  mapDispatchToProps
)(Output);
