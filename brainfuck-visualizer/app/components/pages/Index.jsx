const React = require('react');

const Settings = require('../../containers/Settings');
const Tape = require('../../containers/Tape');

const Page = require('../Page');

const Editor = () => (
  <Page>
    <Tape />
    <Settings />
  </Page>
);

module.exports = Editor;
