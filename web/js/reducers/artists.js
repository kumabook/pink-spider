import { combineReducers } from 'redux';
import {
  index,
  show,
} from '../actions/artist';

const total = (state = 0, action) => {
  switch (action.type) {
    case index.succeeded:
      return action.payload.total;
    default:
      return state;
  }
};

const items = (state = [], action) => {
  switch (action.type) {
    case index.succeeded:
      return action.payload.items;
    default:
      return state;
  }
};

const item = (state = {}, action) => {
  switch (action.type) {
    case show.succeeded:
      return action.payload;
    default:
      return state;
  }
};


export default combineReducers({
  total,
  items,
  item,
});
