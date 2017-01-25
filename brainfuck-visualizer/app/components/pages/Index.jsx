const React = require('react');

const Settings = require('../../containers/Settings');
const Interpreter = require('../../containers/Interpreter');

const Page = require('../Page');

const Editor = () => (
  <Page>
    <Interpreter />
    <Settings />
  </Page>
);

module.exports = Editor;
