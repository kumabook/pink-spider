import entry from '../api/entry';
import track from '../api/track';

export const toggleDrawler = () => ({ type: 'TOGGLE_DRAWLER' });

export const receiveEntries = entries => ({
  type: 'RECEIVE_ENTRIES',
  items: entries,
});

export const fetchEntries = () => (dispatch) => {
  entry.index().then((entries) => {
    dispatch(receiveEntries(entries));
  });
};

export const receiveTracks = tracks => ({
  type: 'RECEIVE_TRACKS',
  items: tracks,
});

export const fetchTracks = () => (dispatch) => {
  track.index().then((tracks) => {
    dispatch(receiveTracks(tracks));
  });
};
