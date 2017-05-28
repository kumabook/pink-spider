import createHistory        from 'history/createHashHistory';
import { routerMiddleware } from 'react-router-redux';
import createSagaMiddleware from 'redux-saga';
import {
  applyMiddleware,
  createStore,
} from 'redux';
import reducers from '../js/reducers';

export default () => {
  const history = createHistory();
  const sagaMiddleware = createSagaMiddleware();
  const middleware = routerMiddleware(history);
  return createStore(reducers, applyMiddleware(middleware, sagaMiddleware));
};
