import createHistory        from 'history/createHashHistory';
import { routerMiddleware } from 'react-router-redux';
import thunk                from 'redux-thunk';
import {
  applyMiddleware,
  createStore,
} from 'redux';
import reducers from '../js/reducers';

export default () => {
  const history = createHistory();
  const middleware = routerMiddleware(history);
  return createStore(reducers, applyMiddleware(middleware, thunk));
};
