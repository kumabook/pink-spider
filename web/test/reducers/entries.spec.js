import assert              from 'assert';
import reducer, { Status } from '../../js/reducers/entries';

describe('entries reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      {
        total: 0,
        items: [],
      }
    );
  });
});
