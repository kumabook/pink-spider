import entry    from '../api/entry';
import track    from '../api/track';
import playlist from '../api/playlist';

export const toggleDrawler = () => ({ type: 'TOGGLE_DRAWLER' });

export const receiveEntries = entries => ({
  type:    'RECEIVE_ENTRIES',
  page:    entries.page,
  perPage: entries.per_page,
  total:   entries.total,
  items:   entries.items,
});

export const fetchEntries = (page = 0, perPage = 10) => (dispatch) => {
  dispatch({ type: 'FETCH_ENTRIES', page, perPage });
  return entry.index(page, perPage).then((entries) => {
    dispatch(receiveEntries(entries));
  });
};

export const receiveTracks = (tracks, entryId) => ({
  type:    'RECEIVE_TRACKS',
  entryId,
  page:    tracks.page,
  perPage: tracks.per_page,
  total:   tracks.total,
  items:   tracks.items,
});

export const fetchTracks = (page = 0, perPage = 10, entryId = null) => (dispatch) => {
  dispatch({ type: 'FETCH_TRACKS', page, perPage, entryId });
  const promise = entryId ? track.indexByEntry(entryId, page, perPage) :
        track.index(page, perPage);
  return promise.then(tracks => dispatch(receiveTracks(tracks, entryId)));
};

export const fetchTrack = trackId => (dispatch) => {
  dispatch({ type: 'FETCH_TRACK' });
  return track.show(trackId).then(item => dispatch({ type: 'RECEIVE_TRACK', item }));
};

export const updateTrack = trackId => dispatch =>
  track.update(trackId).then(() => dispatch({ type: 'UPDATE_TRACK' }));

export const fetchPlaylist = playlistId => (dispatch) => {
  dispatch({ type: 'FETCH_PLAYLIST' });
  return playlist.show(playlistId).then(item => dispatch({ type: 'RECEIVE_PLAYLIST', item }));
};

export const receivePlaylists = (playlists, entryId) => ({
  type:    'RECEIVE_PLAYLISTS',
  entryId,
  page:    playlists.page,
  perPage: playlists.per_page,
  total:   playlists.total,
  items:   playlists.items,
});

export const fetchPlaylists = (page = 0, perPage = 10, entryId = null) => (dispatch) => {
  dispatch({ type: 'FETCH_PLAYLISTS', page, perPage, entryId });
  const promise = entryId ? playlist.indexByEntry(entryId, page, perPage) :
        playlist.index(page, perPage);
  return promise.then(playlists => dispatch(receivePlaylists(playlists, entryId)));
};
