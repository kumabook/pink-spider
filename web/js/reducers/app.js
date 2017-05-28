import { combineReducers } from 'redux';

const drawlerIsOpen = (state = false, action) => {
  if (action.type === 'TOGGLE_DRAWLER') {
    return !state;
  }
  return state;
};

const progress = (state = false, action) => {
  switch (action.type) {
    case 'SHOW_PROGRESS':
      return true;
    case 'HIDE_PROGRESS':
      return false;
    default:
      return state;
  }
};

const message = (state = '', action) => {
  switch (action.type) {
    case 'SHOW_MESSAGE':
      return action.payload;
    case 'CLOSE_MESSAGE':
      return '';
    default:
      return state;
  }
};

const titleReducer = (state = '') => state;

export default combineReducers({
  drawlerIsOpen,
  message,
  progress,
  title: titleReducer,
});
