import React                from 'react';
import ReactDOM             from 'react-dom';
import MuiThemeProvider     from 'material-ui/styles/MuiThemeProvider';
import { Provider }         from 'react-redux';
import { Route, Switch }    from 'react-router-dom';
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
import createSagaMiddleware from 'redux-saga';
import App                  from './containers/App';
import FeedList             from './containers/FeedList';
import EntryList            from './containers/EntryList';
import EntryDetail          from './containers/EntryDetail';
import TrackList            from './containers/TrackList';
import TrackDetail          from './containers/TrackDetail';
import PlaylistList         from './containers/PlaylistList';
import PlaylistDetail       from './containers/PlaylistDetail';
import AlbumList            from './containers/AlbumList';
import AlbumDetail          from './containers/AlbumDetail';
import ArtistList           from './containers/ArtistList';
import reducers             from './reducers';
import rootSaga             from './sagas';

injectTapEventPlugin();

const history = createHistory();
const sagaMiddleware = createSagaMiddleware();
const store = createStore(reducers, applyMiddleware(sagaMiddleware,
                                                    routerMiddleware(history)));
sagaMiddleware.run(rootSaga);
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
            <Route path="/entries/:entry_id/tracks" component={TrackList} />
            <Route path="/entries/:entry_id/playlists" component={PlaylistList} />
            <Route path="/entries/:entry_id" component={EntryDetail} />
            <Route path="/entries" component={EntryList} />
            <Route path="/artists" component={ArtistList} />
            <Route path="/feeds/:feed_id/entries" component={EntryList} />
            <Route path="/feeds" component={FeedList} />
            <Route path="/" component={EntryList} />
          </Switch>
        </App>
      </ConnectedRouter>
    </Provider>
  </MuiThemeProvider>,
  document.getElementById('container'));
