import {
  router,
  createHashHistory,
} from 'redux-saga-router';
import { fork, put }                   from 'redux-saga/effects';
import { creators as feedActions }     from '../actions/feed';
import { creators as entryActions }    from '../actions/entry';
import { creators as trackActions }    from '../actions/track';
import { creators as albumActions }    from '../actions/album';
import { creators as playlistActions } from '../actions/playlist';
import { creators as artistActions }   from '../actions/artist';
import {
  getPage,
  getPerPage,
  getQuery,
} from '../utils/url';

const history = createHashHistory();

const routes = {
  '/feeds': function* fetchFeeds() {
    const { search } = history.location;
    yield put(feedActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
    }));
  },
  '(?:/feeds/:feedId)?/entries': function* fetchEntriesOfFeed({ feedId }) {
    const { search } = history.location;
    yield put(entryActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
      feedId,
    }));
  },
  '/entries': function* fetchEntries() {
    const { search } = history.location;
    yield put(entryActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
    }));
  },
  '/entries/:entryId': function* fetchEntry({ entryId }) {
    yield put(entryActions.show.start(entryId));
  },
  '(?:/entries/:entryId)?/tracks': function* fetchTracks({ entryId }) {
    const { search } = history.location;
    yield put(trackActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
      entryId,
    }));
  },
  '/tracks/:trackId': function* fetchTrack({ trackId }) {
    yield put(trackActions.show.start(trackId));
  },
  '(?:/entries/:entryId)?/albums': function* fetchAlbums({ entryId }) {
    const { search } = history.location;
    yield put(albumActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
      entryId,
    }));
  },
  '/albums/:albumId': function* fetchAlbum({ albumId }) {
    yield put(albumActions.show.start(albumId));
  },
  '(?:/entries/:entryId)?/playlists': function* fetchPlaylists({ entryId }) {
    const { search } = history.location;
    yield put(playlistActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
      entryId,
    }));
  },
  '/playlists/:playlistId': function* fetchPlaylist({ playlistId }) {
    yield put(playlistActions.show.start(playlistId));
  },
  '/artists': function* fetchArtists() {
    const { search } = history.location;
    yield put(artistActions.index.start({
      page:    getPage(search),
      perPage: getPerPage(search),
      query:   getQuery(search),
    }));
  },
  '/artists/:artistId': function* fetchArtist({ artistId }) {
    yield put(artistActions.show.start(artistId));
  },
};

export default function* routerSaga() {
  yield fork(router, history, routes);
}
