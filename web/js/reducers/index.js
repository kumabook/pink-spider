import { combineReducers } from 'redux';
import { routerReducer }   from 'react-router-redux';
import app                 from './app';
import entries             from './entries';
import track               from './track';
import tracks              from './tracks';

const rootReducer = combineReducers({
  routing: routerReducer,
  app,
  entries,
  track,
  tracks,
});

export default rootReducer;
