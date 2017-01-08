import { hashHistory }      from 'react-router';
import { routerMiddleware } from 'react-router-redux';
import thunk                from 'redux-thunk';
import {
  applyMiddleware,
  createStore,
} from 'redux';
import reducers from '../js/reducers';

export default () => {
  const middleware = routerMiddleware(hashHistory);
  return createStore(reducers, applyMiddleware(middleware, thunk));
};
