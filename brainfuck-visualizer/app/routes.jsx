const Index = require('./components/pages/Index');

const routes = Object.freeze({
  index: Object.freeze({
    //path: 'editor',
    pattern: /^\//,
    component: Index,
    onEnter: Index.onPageEnter,
    onLeave: Index.onPageLeave,
  }),
});

module.exports = routes;
