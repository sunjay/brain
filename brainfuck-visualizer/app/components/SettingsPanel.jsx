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

const SettingsPanel = ({command, file, onSubmit}) => {
  let commandInput, fileInput;

  const submit = (e) => {
    e.preventDefault();
    const command = commandInput.value;
    const file = fileInput.value;
    if (!command || !command.trim() || !file || !file.trim()) {
      return;
    }
    onSubmit(command, file);
  };
  return (
    <Panel>
      <form onSubmit={submit}>
        <div className={formGroup}>
          <label className={formLabel}>Brainfuck Command:</label>
          <input type='text' defaultValue={command} className={formControl}
            ref={(node) => commandInput = node}/>
        </div>

        <div className={formGroup}>
          <label className={formLabel}>Brainfuck File:</label>
          <input type='text' defaultValue={file} className={formControl}
            ref={(node) => fileInput = node}/>
        </div>

        <div className={classNames(formGroup, formGroupRight)}>
          <Button style='primary' type='submit' block>
            Start
          </Button>
        </div>
      </form>
    </Panel>
  );
};

SettingsPanel.propTypes = {
  command: React.PropTypes.string,
  file: React.PropTypes.string,
};

module.exports = SettingsPanel;
