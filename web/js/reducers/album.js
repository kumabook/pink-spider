import { combineReducers } from 'redux';
import { LOCATION_CHANGE } from 'react-router-redux';

export const Status = {
  Normal:   'Normal',
  Dirty:    'Dirty',
  Fetching: 'Fetching',
};

const status = (state = Status.Dirty, action) => {
  switch (action.type) {
    case 'FETCH_ALBUM':
      return Status.Fetching;
    case 'RECEIVE_ALBUM':
      return Status.Normal;
    case 'UPDATE_ALBUM':
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
    case 'RECEIVE_ALBUM':
      return action.item;
    default:
      return state;
  }
};

export default combineReducers({
  status,
  item,
});
