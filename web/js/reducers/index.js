import { combineReducers } from 'redux';
import { routerReducer }   from 'react-router-redux';
import app                 from './app';
import entries             from './entries';
import track               from './track';
import tracks              from './tracks';
import playlist            from './playlist';
import playlists           from './playlists';

const rootReducer = combineReducers({
  routing: routerReducer,
  app,
  entries,
  track,
  tracks,
  playlist,
  playlists,
});

export default rootReducer;
