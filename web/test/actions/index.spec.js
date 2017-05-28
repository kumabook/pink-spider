import assert               from 'assert';
import configureMockStore   from 'redux-mock-store';
import createSagaMiddleware from 'redux-saga';
import axios                from 'axios';
import MockAdapter          from 'axios-mock-adapter';
import * as actions         from '../../js/actions/track';

const mock = new MockAdapter(axios);
axios.defaults.baseURL = 'http://0.0.0.0:8080';

const sagaMiddleware = createSagaMiddleware();
const middlewares = [sagaMiddleware];
const mockStore   = configureMockStore(middlewares);

describe('actions', () => {
  afterEach(() => {
    mock.reset();
  });
});
