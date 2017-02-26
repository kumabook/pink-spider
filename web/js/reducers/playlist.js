import { combineReducers } from 'redux';
import { LOCATION_CHANGE } from 'react-router-redux';

export const Status = {
  Normal:   'Normal',
  Dirty:    'Dirty',
  Fetching: 'Fetching',
};

const status = (state = Status.Dirty, action) => {
  switch (action.type) {
    case 'FETCH_PLAYLIST':
      return Status.Fetching;
    case 'RECEIVE_PLAYLIST':
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

const item = (state = {}, action) => {
  switch (action.type) {
    case 'RECEIVE_PLAYLIST':
      return action.item;
    default:
      return state;
  }
};

export default combineReducers({
  status,
  item,
});