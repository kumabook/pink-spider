import assert       from 'assert';
import parseIntOr   from '../../js/utils/parseIntOr';

describe('parseIntOr', () => {
  it('should return parsed int value or default value', () => {
    assert.equal(1, parseIntOr('1', 0));
    assert.equal(-1, parseIntOr('test', -1));
    assert.equal(-1, parseIntOr(null, -1));
  });
});
