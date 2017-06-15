import {
  router,
  createHashHistory,
} from 'redux-saga-router';
import { fork, put }                   from 'redux-saga/effects';
import { creators as feedActions }    from '../actions/feed';
import { creators as entryActions }    from '../actions/entry';
import { creators as trackActions }    from '../actions/track';
import { creators as albumActions }    from '../actions/album';
import { creators as playlistActions } from '../actions/playlist';
import { creators as artistActions }   from '../actions/artist';
import { defaultPerPage }              from '../config';
import parseIntOr                      from '../utils/parseIntOr';

const history = createHashHistory();
export const getSerachParams = () => new URLSearchParams(history.location.search);
export const getPage = () => parseIntOr(getSerachParams().get('page'), 0);
export const getPerPage = () => parseIntOr(getSerachParams().get('per_page'), defaultPerPage);

const routes = {
  '/feeds': function* fetchFeeds() {
    yield put(feedActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
    }));
  },
  '(?:/feeds/:feedId)?/entries': function* fetchEntriesOfFeed({ feedId }) {
    yield put(entryActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
      feedId,
    }));
  },
  '/entries': function* fetchEntries() {
    yield put(entryActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
    }));
  },
  '(?:/entries/:entryId)?/tracks': function* fetchTracks({ entryId }) {
    yield put(trackActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
      entryId,
    }));
  },
  '/tracks/:trackId': function* fetchTrack({ trackId }) {
    yield put(trackActions.show.start(trackId));
  },
  '(?:/entries/:entryId)?/albums': function* fetchAlbums({ entryId }) {
    yield put(albumActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
      entryId,
    }));
  },
  '/albums/:albumId': function* fetchAlbum({ albumId }) {
    yield put(albumActions.show.start(albumId));
  },
  '(?:/entries/:entryId)?/playlists': function* fetchPlaylists({ entryId }) {
    yield put(playlistActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
      entryId,
    }));
  },
  '/playlists/:playlistId': function* fetchPlaylist({ playlistId }) {
    yield put(playlistActions.show.start(playlistId));
  },
  '/artists': function* fetchArtists() {
    yield put(artistActions.index.start({
      page:    getPage(),
      perPage: getPerPage(),
    }));
  },
  '/artists/:artistId': function* fetchArtist({ artistId }) {
    yield put(artistActions.show.start(artistId));
  },
};

export default function* routerSaga() {
  yield fork(router, history, routes);
}
