const React = require('react');

const Page = require('../Page');

const {
  notFound,
} = require('../../../scss/pages/notFound.scss');

const NotFound = () => (
  <Page>
    <div className={notFound}>
      <h1>Brainfuck Visualizer</h1>

      <h2>Error Loading Screen</h2>
      <p>This screen could not be loaded.
        This is likely an error on our end. Sorry about that!
        Please close this window and try again.</p>
    </div>
  </Page>
);

module.exports = NotFound;
