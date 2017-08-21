import { combineReducers } from 'redux';
import {
  index,
  show,
  update,
  preview,
} from '../actions/entry';

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
    case update.succeeded:
      return action.payload;
    default:
      return state;
  }
};

const previewType = (state = 'hidden', action) => {
  switch (action.type) {
    case preview:
      return action.payload;
    default:
      return state;
  }
};

export default combineReducers({
  total,
  items,
  item,
  previewType,
});
