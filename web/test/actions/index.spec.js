import assert             from 'assert';
import configureMockStore from 'redux-mock-store';
import thunk              from 'redux-thunk';
import axios              from 'axios';
import MockAdapter        from 'axios-mock-adapter';
import * as actions       from '../../js/actions';

const mock = new MockAdapter(axios);
axios.defaults.baseURL = 'http://0.0.0.0:8080';

const middlewares = [thunk];
const mockStore   = configureMockStore(middlewares);

describe('actions', () => {
  afterEach(() => {
    mock.reset();
  });
  describe('.fetchEntries', () => {
    it('creates RECEIVE_ENTRIES when fetching entries has been done', () => {
      mock.onGet('/v1/entries').reply(200, { items: [], page: 0, per_page: 10, total: 0 });
      const expectedActions = [
        { type: 'FETCH_ENTRIES', page: 0, perPage: 10 },
        { type: 'RECEIVE_ENTRIES', page: 0, perPage: 10, total: 0, items: []},
      ];
      const store = mockStore();
      return store.dispatch(actions.fetchEntries()).then(() => {
        assert.deepEqual(store.getActions(), expectedActions);
      });
    });
  });
  describe('.fetchTracks', () => {
    it('creates RECEIVE_TRACKS when fetching tracks has been done', () => {
      mock.onGet('/v1/tracks').reply(200, { items: [], page: 0, per_page: 10, total: 0 });
      const expectedActions = [
        { type: 'FETCH_TRACKS', entryId: null, page: 0, perPage: 10 },
        { type: 'RECEIVE_TRACKS', entryId: null, page: 0, perPage: 10, total: 0, items: []},
      ];
      const store = mockStore();
      return store.dispatch(actions.fetchTracks()).then(() => {
        assert.deepEqual(store.getActions(), expectedActions);
      });
    });
  });
  describe('.fetchTrack', () => {
    it('creates RECEIVE_TRACK when fetching a track has been done', () => {
      const track = {
        id: 'track_id',
      };
      mock.onGet('/v1/tracks/track_id').reply(200, track);
      const expectedActions = [
        { type: 'FETCH_TRACK' },
        { type: 'RECEIVE_TRACK', item: track },
      ];
      const store = mockStore();
      return store.dispatch(actions.fetchTrack('track_id')).then(() => {
        assert.deepEqual(store.getActions(), expectedActions);
      });
    });
  });
  describe('.updateTrack', () => {
    it('creates UPDATE_TRACK when fetching a track has been done', () => {
      const track = {
        id: 'track_id',
      };
      mock.onPost('/v1/tracks/track_id').reply(200, track);
      const expectedActions = [{ type: 'UPDATE_TRACK' }];
      const store = mockStore();
      return store.dispatch(actions.updateTrack('track_id')).then(() => {
        assert.deepEqual(store.getActions(), expectedActions);
      });
    });
  });
});
