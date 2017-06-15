import { combineReducers } from 'redux';
import { routerReducer }   from 'react-router-redux';
import app                 from './app';
import feeds               from './feeds';
import entries             from './entries';
import tracks              from './tracks';
import playlists           from './playlists';
import albums              from './albums';
import artists             from './artists';

const rootReducer = combineReducers({
  router: routerReducer,
  app,
  feeds,
  entries,
  tracks,
  playlists,
  albums,
  artists,
});

export default rootReducer;
