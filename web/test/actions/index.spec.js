import axios       from 'axios';
import MockAdapter from 'axios-mock-adapter';

const mock = new MockAdapter(axios);

describe('actions', () => {
  afterEach(() => {
    mock.reset();
  });
});
