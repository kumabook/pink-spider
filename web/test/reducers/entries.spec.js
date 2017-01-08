import assert              from 'assert';
import reducer, { Status } from '../../js/reducers/entries';

describe('entries reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      {
        status:  Status.Dirty,
        page:    0,
        perPage: 10,
        total:   0,
        items:   [],
      }
    );
  });

  it('should handle FETCH_ENTRIES', () => {
    const fetchAction = { type: 'FETCH_ENTRIES' };
    assert.deepEqual(
      reducer(undefined, fetchAction),
      {
        status:   Status.Fetching,
        page:     0,
        perPage: 10,
        total:    0,
        items:   [],
      });
  });

  it('should handle RECEIVE_ENTRIES', () => {
    const receiveAction = {
      type:    'RECEIVE_ENTRIES',
      page:    0,
      perPage: 10,
      total:   1,
      items:   [{ id: 'entry1'}]
    };
    assert.deepEqual(
      reducer(undefined, receiveAction),
      {
        status:   Status.Normal,
        page:     0,
        perPage: 10,
        total:    1,
        items:   [{ id: 'entry1' }],
      });
  });
});
