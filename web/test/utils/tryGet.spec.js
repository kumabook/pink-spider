import assert from 'assert';
import tryGet from '../../js/utils/tryGet';

describe('tryGet', () => {
  it('should return object property or default value', () => {
    assert.equal(1, tryGet({ key1: 1}, 'key1', 0));
    assert.equal(0, tryGet({ key1: 1}, 'key2', 0));
    assert.equal(0, tryGet(null      , 'key1', 0));
  });
});
