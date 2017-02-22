import { combineReducers } from 'redux';
import { LOCATION_CHANGE } from 'react-router-redux';
import parseIntOr          from '../utils/parseIntOr';

export const Status = {
  Normal:   'Normal',
  Dirty:    'Dirty',
  Fetching: 'Fetching',
};

const status = (state = Status.Dirty, action) => {
  switch (action.type) {
    case 'FETCH_PLAYLISTS':
      return Status.Fetching;
    case 'RECEIVE_PLAYLISTSS':
      return Status.Normal;
    case 'UPDATE_PLAYLIST':
      return Status.Dirty;
    case LOCATION_CHANGE:
      if (state !== Status.Fetching) {
        return Status.Dirty;
      }
      return state;
    default:
      return state;
  }
};

const entryId = (state = '', action) => {
  switch (action.type) {
    case 'FETCH_PLAYLISTS':
      return action.entryId;
    case 'RECEIVE_PLAYLISTS':
      return action.entryId;
    default:
      return state;
  }
};

const page = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_PLAYLISTS':
      return action.page;
    case LOCATION_CHANGE:
      return parseIntOr(action.payload.query.page, 0);
    default:
      return state;
  }
};

const perPage = (state = 10, action) => {
  switch (action.type) {
    case 'RECEIVE_PLAYLISTS':
      return action.perPage;
    case LOCATION_CHANGE:
      return parseIntOr(action.payload.query.per_page, 10);
    default:
      return state;
  }
};

const total = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_PLAYLISTS':
      return action.total;
    default:
      return state;
  }
};

const items = (state = [], action) => {
  switch (action.type) {
    case 'RECEIVE_PLAYLISTS':
      return action.items;
    default:
      return state;
  }
};

export default combineReducers({
  status,
  entryId,
  items,
  page,
  perPage,
  total,
});
