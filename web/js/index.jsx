import React                   from 'react';
import ReactDOM                from 'react-dom';
import MuiThemeProvider        from 'material-ui/styles/MuiThemeProvider';
import { Provider }            from 'react-redux';
import { Router, hashHistory } from 'react-router';
import thunk                   from 'redux-thunk';
import injectTapEventPlugin    from 'react-tap-event-plugin';
import {
  applyMiddleware,
  createStore,
} from 'redux';
import {
  syncHistoryWithStore,
  routerMiddleware,
} from 'react-router-redux';

import App            from './containers/App';
import EntryList      from './containers/EntryList';
import TrackList      from './containers/TrackList';
import TrackDetail    from './containers/TrackDetail';
import PlaylistList   from './containers/PlaylistList';
import PlaylistDetail from './containers/PlaylistDetail';
import AlbumList      from './containers/AlbumList';
import AlbumDetail    from './containers/AlbumDetail';
import reducers       from './reducers';

injectTapEventPlugin();

const middleware = routerMiddleware(hashHistory);
const store = createStore(reducers, applyMiddleware(middleware, thunk));

const history = syncHistoryWithStore(hashHistory, store);
const routes = [{
  path:        '/',
  component:   App,
  childRoutes: [
    { path: 'tracks/:track_id', component: TrackDetail },
    { path: 'tracks', component: TrackList },
    { path: 'playlists/:playlist_id', component: PlaylistDetail },
    { path: 'playlists', component: PlaylistList },
    { path: 'albums/:album_id', component: AlbumDetail },
    { path: 'albums', component: AlbumList },
    { path: 'entries', component: EntryList },
    { path: 'entries/:entry_id/tracks', component: TrackList },
    { path: 'entries/:entry_id/playlists', component: PlaylistList },
    { path: 'entries/:entry_id/albums', component: AlbumList },
    { path: '*', component: EntryList },
  ],
}];
ReactDOM.render(
  <MuiThemeProvider>
    <Provider store={store}>
      <Router history={history} routes={routes} />
    </Provider>
  </MuiThemeProvider>,
  document.getElementById('container'));
