import React    from 'react';
import ReactDOM from 'react-dom';
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import { applyMiddleware, createStore } from 'redux';
import { Provider } from 'react-redux';
import { Router, hashHistory } from 'react-router';
import thunk from 'redux-thunk';
import { syncHistoryWithStore, routerMiddleware } from 'react-router-redux';
import injectTapEventPlugin from 'react-tap-event-plugin';

import App       from './containers/App';
import EntryList  from './containers/EntryList';
import TrackList from './containers/TrackList';
import reducers  from './reducers';

injectTapEventPlugin();

const middleware = routerMiddleware(hashHistory);
const store = createStore(reducers, applyMiddleware(middleware, thunk));

const history = syncHistoryWithStore(hashHistory, store);
const routes = [{
  path: '/',
  component: App,
  childRoutes: [
    { path: 'tracks', component: TrackList },
    { path: 'entries', component: EntryList },
    { path: 'entries/:entry_id/tracks', component: TrackList },
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
