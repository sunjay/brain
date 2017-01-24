const {expect} = require('chai');

const {createRecord} = require('../../app/models/model');

describe('createRecord', () => {
  const constant = 1234;
  const otherField = 'abcdef';
  const staticField = 'CONSTANT';
  let TestRecord;

  beforeEach(() => {
    TestRecord = createRecord({
      [staticField]: constant,
    }, (constants) => ({
      constant: constants[staticField],
      otherField: otherField,
    }));
  });

  it('should correctly assign static properties', () => {
    expect(TestRecord[staticField]).to.equal(constant);
  });

  it('should have correct default field values', () => {
    const instance = new TestRecord();
    expect(instance.constant).to.equal(constant);
    expect(instance.otherField).to.equal(otherField);
  });

  it('should produce a record class that is immutable', () => {
    const instance = new TestRecord();

    const constant2 = 'ADIFIFD';
    const otherField2 = false;
    const instance2 = instance
      .set('constant', constant2)
      .set('otherField', otherField2);

    expect(instance.constant).to.equal(constant);
    expect(instance.otherField).to.equal(otherField);

    expect(instance2.constant).to.equal(constant2);
    expect(instance2.otherField).to.equal(otherField2);
  });
});
