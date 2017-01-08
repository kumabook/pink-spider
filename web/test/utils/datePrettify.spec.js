import assert       from 'assert';
import moment       from 'moment';
import datePrettify from '../../js/utils/datePrettify';

describe('datePrettify', () => {
  describe('.dateString', () => {
    it('should return pretty date string', () => {
      assert.equal('2016-01-06 01:00:00', datePrettify(moment([2016, 0, 6, 1, 0, 0, 0])));
    });
  });
});
