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

export const fetchTracks = (page = 0, perPage = 10) => (dispatch) => {
  track.index(page, perPage).then((tracks) => {
    dispatch(receiveTracks(tracks));
  });
};
