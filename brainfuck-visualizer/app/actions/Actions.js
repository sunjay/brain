const {Set} = require('immutable');

const Actions = {
  actions: new Set(),
  actionCreators: new Set(),
  register(name) {
    if (this.actions.has(name)) {
      throw new Error(`Attempt to register existing action '${name}'`);
    }
    this.actions = this.actions.add(name);
    return name;
  },
  registerActionCreator(type, fields = []) {
    if (this.actionCreators.has(type)) {
      throw new Error(`Attempt to register existing action creator '${type}'`);
    }
    this.actionCreators = this.actionCreators.add(type);

    if (typeof fields === 'function') {
      return fields;
    }
    return this.createActionCreator(type, fields);
  },
  createActionCreator(type, fields = []) {
    return (...args) => {
      return this.createActionFromFields(type, fields, args);
    };
  },
  createActionFromFields(type, fields, values) {
    return this.createAction(type, fields.reduce((data, field, i) => (
      Object.assign({}, data, {[field]: values[i]})
    ), {}));
  },
  createAction(type, data = {}) {
    return Object.assign({}, data, {type});
  },
};

module.exports = Actions;
