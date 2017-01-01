import entry from '../api/entry';
import track from '../api/track';

export const toggleDrawler = () => ({ type: 'TOGGLE_DRAWLER' });

export const receiveEntries = entries => ({
  type: 'RECEIVE_ENTRIES',
  page: entries.page,
  perPage: entries.per_page,
  total: entries.total,
  items: entries.items,
});

export const fetchEntries = (page = 0, perPage = 10) => (dispatch) => {
  dispatch({ type: 'FETCH_ENTRIES', page, perPage });
  entry.index(page, perPage).then((entries) => {
    dispatch(receiveEntries(entries));
  });
};

export const receiveTracks = tracks => ({
  type: 'RECEIVE_TRACKS',
  page: tracks.page,
  perPage: tracks.per_page,
  total: tracks.total,
  items: tracks.items,
});

export const fetchTracks = (page = 0, perPage = 10, entryId = null) =>
  (dispatch) => {
    dispatch({ type: 'FETCH_TRACKS', page, perPage, entryId });
    const promise = entryId ? track.indexByEntry(entryId, page, perPage) :
                              track.index(page, perPage);
    promise.then((tracks) => {
      dispatch(receiveTracks(tracks));
    });
  };
