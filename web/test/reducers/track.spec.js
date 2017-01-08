import assert              from 'assert';
import reducer, { Status } from '../../js/reducers/track';

describe('track reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      { status: Status.Dirty, item: {} }
    );
  });

  it('should handle FETCH_TRACK', () => {
    const fetchAction = { type: 'FETCH_TRACK' };
    assert.deepEqual(
      reducer(undefined, fetchAction),
      { status: Status.Fetching, item: {}}
    );
  });

  it('should handle UPDATE_TRACK', () => {
    const fetchAction = { type: 'UPDATE_TRACK' };
    assert.deepEqual(
      reducer(undefined, fetchAction),
      { status: Status.Dirty, item: {}}
    );
  });

  it('should handle RECEIVE_TRACK', () => {
    const fetchAction = { type: 'RECEIVE_TRACK', item: { id: 'track1' }};
    assert.deepEqual(
      reducer(undefined, fetchAction),
      { status: Status.Normal, item: { id: 'track1' }}
    );
  });

});
