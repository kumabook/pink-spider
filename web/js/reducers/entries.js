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
    case 'FETCH_ENTRIES':
      return Status.Fetching;
    case 'RECEIVE_ENTRIES':
      return Status.Normal;
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
    case 'RECEIVE_ENTRIES':
      return action.page;
    case LOCATION_CHANGE:
      return parseIntOr(action.payload.query.page, 0);
    default:
      return state;
  }
};

const perPage = (state = 10, action) => {
  switch (action.type) {
    case 'RECEIVE_ENTRIES':
      return action.perPage;
    case LOCATION_CHANGE:
      return parseIntOr(action.payload.query.per_page, 10);
    default:
      return state;
  }
};

const total = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_ENTRIES':
      return action.total;
    default:
      return state;
  }
};

const items = (state = [], action) => {
  switch (action.type) {
    case 'RECEIVE_ENTRIES':
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
