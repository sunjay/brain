const React = require('react');
const classNames = require('classnames');

const {
  formGroup,
  formGroupRight,
  formLabel,
  formControl,
} = require('../../scss/components/forms.scss');

const Panel = require('./Panel');
const Button = require('./Button');

const SettingsPanel = ({command, file}) => (
  <Panel>
    <div className={formGroup}>
      <label className={formLabel}>Brainfuck Command:</label>
      <input type='text' value={command} className={formControl} />
    </div>

    <div className={formGroup}>
      <label className={formLabel}>Brainfuck file:</label>
      <input type='text' value={file} className={formControl} />
    </div>

    <div className={classNames(formGroup, formGroupRight)}>
      <Button style='primary' block>Run</Button>
    </div>
  </Panel>
);

SettingsPanel.propTypes = {
  className: React.PropTypes.string,
};

module.exports = SettingsPanel;
