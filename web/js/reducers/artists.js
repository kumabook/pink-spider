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
    case 'FETCH_ARTISTS':
      return Status.Fetching;
    case 'RECEIVE_ARTISTS':
      return Status.Normal;
    case 'UPDATE_ARTIST':
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

const page = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_ARTISTS':
      return action.page;
    case LOCATION_CHANGE: {
      const query = new URLSearchParams(action.payload.search);
      return parseIntOr(query.get('page'), 0);
    }
    default:
      return state;
  }
};

const perPage = (state = 10, action) => {
  switch (action.type) {
    case 'RECEIVE_ARTISTS':
      return action.perPage;
    case LOCATION_CHANGE: {
      const query = new URLSearchParams(action.payload.search);
      return parseIntOr(query.get('per_page'), 10);
    }
    default:
      return state;
  }
};

const total = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_ARTISTS':
      return action.total;
    default:
      return state;
  }
};

const items = (state = [], action) => {
  switch (action.type) {
    case 'RECEIVE_ARTISTS':
      return action.items;
    default:
      return state;
  }
};

export default combineReducers({
  status,
  items,
  page,
  perPage,
  total,
});
