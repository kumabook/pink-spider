import assert  from 'assert';
import reducer from '../../js/reducers/app';

describe('app reducer', () => {
  it('should return the initial state', () => {
    assert.deepEqual(
      reducer(undefined, {}),
      { drawlerIsOpen: false }
    );
  });

  it('should handle TOGGLE_DRAWLER', () => {
    assert.deepEqual(
      reducer({ drawlerIsOpen: false }, { type: 'TOGGLE_DRAWLER' }),
      { drawlerIsOpen: true }
    );
    assert.deepEqual(
      reducer({ drawlerIsOpen: true }, { type: 'TOGGLE_DRAWLER' }),
      { drawlerIsOpen: false }
    );
  });
});
