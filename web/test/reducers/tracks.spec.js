import assert              from 'assert';
import reducer, { Status } from '../../js/reducers/tracks';

describe('tracks reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      {
        status:  Status.Dirty,
        entryId: '',
        page:    0,
        perPage: 10,
        total:   0,
        items:   [],
      }
    );
  });

  it('should handle FETCH_TRACKS', () => {
    const fetchAction = { type: 'FETCH_TRACKS', entryId: 'entry1' };
    assert.deepEqual(
      reducer(undefined, fetchAction),
      {
        status:   Status.Fetching,
        entryId:  'entry1',
        page:     0,
        perPage: 10,
        total:    0,
        items:   [],
      });
  });

  it('should handle RECEIVE_TRACKS', () => {
    const receiveAction = {
      type:    'RECEIVE_TRACKS',
      entryId: 'entry1',
      page:    0,
      perPage: 10,
      total:   1,
      items:   [{ id: 'track1'}]
    };
    assert.deepEqual(
      reducer(undefined, receiveAction),
      {
        status:   Status.Normal,
        entryId:  'entry1',
        page:     0,
        perPage: 10,
        total:    1,
        items:   [{ id: 'track1' }],
      });
  });
});
