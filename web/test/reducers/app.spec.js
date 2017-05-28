import assert  from 'assert';
import reducer from '../../js/reducers/app';

describe('app reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      {
        drawlerIsOpen: false,
        message:       '',
        progress:      false,
        title:         '',
      }
    );
  });
});
