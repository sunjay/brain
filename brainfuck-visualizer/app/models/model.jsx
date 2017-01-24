/**
 * This is NOT a base class. Use Immutable records as your base classes.
 * This is a collection of pure functions to avoid repitition in models.
 */

const {Record} = require('immutable');

/**
 * Creates a new Immutable Record class to be extended by a model
 * Avoids exposing an extra constants object that could be accidentally
 * used by someone else instead of the main class itself
 * @param {Object} constants - An object of constants that will become
 *    static members of the created Record
 * @param {Function} recordCreator - recordCreator(constants) will
 *    be called to get the fields and default values for this record
 * @returns {Immutable.Record} The created record
 */
export function createRecord(constants, recordCreator) {
  const NewRecord = Record(recordCreator(constants));
  Object.assign(NewRecord, constants);
  return NewRecord;
}
