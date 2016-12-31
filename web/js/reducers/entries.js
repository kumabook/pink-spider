import { combineReducers } from 'redux';

const page = (state = 0, action) => {
  switch (action.type) {
    case 'RECEIVE_ENTRIES':
      return action.page;
    default:
      return state;
  }
};

const perPage = (state = 20, action) => {
  switch (action.type) {
    case 'RECEIVE_ENTRIES':
      return action.perPage;
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
  items,
  page,
  perPage,
  total,
});
