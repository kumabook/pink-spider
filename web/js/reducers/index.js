import { combineReducers } from 'redux';
import { routerReducer }   from 'react-router-redux';
import app                 from './app';
import entries             from './entries';
import track               from './track';
import tracks              from './tracks';
import playlist            from './playlist';
import playlists           from './playlists';
import album               from './album';
import albums              from './albums';
import artists             from './artists';

const rootReducer = combineReducers({
  routing: routerReducer,
  app,
  entries,
  track,
  tracks,
  playlist,
  playlists,
  album,
  albums,
  artists,
});

export default rootReducer;
