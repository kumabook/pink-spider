import React                from 'react';
import ReactDOM             from 'react-dom';
import MuiThemeProvider     from 'material-ui/styles/MuiThemeProvider';
import { Provider }         from 'react-redux';
import { Route, Switch }    from 'react-router-dom';
import thunk                from 'redux-thunk';
import injectTapEventPlugin from 'react-tap-event-plugin';
import createHistory        from 'history/createHashHistory';
import {
  applyMiddleware,
  createStore,
} from 'redux';
import {
  ConnectedRouter,
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
import ArtistList     from './containers/ArtistList';
import reducers       from './reducers';

injectTapEventPlugin();

const history = createHistory();

const middleware = routerMiddleware(history);
const store = createStore(reducers, applyMiddleware(middleware, thunk));
ReactDOM.render(
  <MuiThemeProvider>
    <Provider store={store}>
      <ConnectedRouter history={history}>
        <App>
          <Switch>
            <Route path="/tracks/:track_id" component={TrackDetail} />
            <Route path="/tracks" component={TrackList} />
            <Route path="/playlists/:playlist_id" component={PlaylistDetail} />
            <Route path="/playlists" component={PlaylistList} />
            <Route path="/albums/:album_id" component={AlbumDetail} />
            <Route path="/albums" component={AlbumList} />
            <Route path="/entries" component={EntryList} />
            <Route path="/entries/:entry_id/tracks" component={TrackList} />
            <Route path="/entries/:entry_id/playlists" component={PlaylistList} />
            <Route path="/entries/:entry_id/albums" component={AlbumList} />
            <Route path="/entries/:entry_id/albums" component={AlbumList} />
            <Route path="/artists" component={ArtistList} />
            <Route path="/" component={EntryList} />
          </Switch>
        </App>
      </ConnectedRouter>
    </Provider>
  </MuiThemeProvider>,
  document.getElementById('container'));
