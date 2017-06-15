import {
  fork,
} from 'redux-saga/effects';
import {
  watchFetchFeeds,
  watchFetchFeed,
  watchUpdateFeed,
} from './feed';
import {
  watchFetchEntries,
  watchFetchEntry,
  watchUpdateEntry,
} from './entry';
import {
  watchFetchTracks,
  watchFetchTrack,
  watchUpdateTrack,
} from './track';
import {
  watchFetchAlbums,
  watchFetchAlbum,
  watchUpdateAlbum,
} from './album';
import {
  watchFetchPlaylists,
  watchFetchPlaylist,
  watchUpdatePlaylist,
} from './playlist';
import {
  watchFetchArtists,
  watchFetchArtist,
  watchUpdateArtist,
} from './artist';
import routerSaga from './router';

export default function* root() {
  yield [
    fork(watchFetchFeeds),
    fork(watchFetchFeed),
    fork(watchUpdateFeed),
    fork(watchFetchEntries),
    fork(watchFetchEntry),
    fork(watchUpdateEntry),
    fork(watchFetchTracks),
    fork(watchFetchTrack),
    fork(watchUpdateTrack),
    fork(watchFetchAlbums),
    fork(watchFetchAlbum),
    fork(watchUpdateAlbum),
    fork(watchFetchPlaylists),
    fork(watchFetchPlaylist),
    fork(watchUpdatePlaylist),
    fork(watchFetchArtists),
    fork(watchFetchArtist),
    fork(watchUpdateArtist),
    fork(routerSaga),
  ];
}
