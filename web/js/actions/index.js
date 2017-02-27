import entry    from '../api/entry';
import track    from '../api/track';
import playlist from '../api/playlist';
import album    from '../api/album';
import artist   from '../api/artist';

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

export const fetchAlbum = albumId => (dispatch) => {
  dispatch({ type: 'FETCH_ALBUM' });
  return album.show(albumId).then(item => dispatch({ type: 'RECEIVE_ALBUM', item }));
};

export const receiveAlbums = (albums, entryId) => ({
  type:    'RECEIVE_ALBUMS',
  entryId,
  page:    albums.page,
  perPage: albums.per_page,
  total:   albums.total,
  items:   albums.items,
});

export const fetchAlbums = (page = 0, perPage = 10, entryId = null) => (dispatch) => {
  dispatch({ type: 'FETCH_ALBUMS', page, perPage, entryId });
  const promise = entryId ? album.indexByEntry(entryId, page, perPage) :
        album.index(page, perPage);
  return promise.then(albums => dispatch(receiveAlbums(albums, entryId)));
};

export const fetchArtist = artistId => (dispatch) => {
  dispatch({ type: 'FETCH_ARTIST' });
  return artist.show(artistId).then(item => dispatch({ type: 'RECEIVE_ARTIST', item }));
};

export const receiveArtists = artists => ({
  type:    'RECEIVE_ARTISTS',
  page:    artists.page,
  perPage: artists.per_page,
  total:   artists.total,
  items:   artists.items,
});

export const fetchArtists = (page = 0, perPage = 10) => (dispatch) => {
  dispatch({ type: 'FETCH_ARTISTS', page, perPage });
  const promise = artist.index(page, perPage);
  return promise.then(artists => dispatch(receiveArtists(artists)));
};
