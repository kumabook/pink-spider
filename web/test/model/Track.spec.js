import assert                 from 'assert';
import {getUrl, getOwnerUrl } from '../../js/model/Track';

describe('Track', () => {
  describe('.getUrl', () => {
    it('should return owner url', () => {
      assert.equal('http://example.com/link', getUrl({
        url: 'http://example.com/link',
      }));
      assert.equal('https://www.youtube.com/watch/?v=abcd', getUrl({
        provider: 'YouTube',
        identifier: 'abcd',
      }));
      assert.equal('https://soundcloud.com/tracks/1234', getUrl({
        provider: 'SoundCloud',
        identifier: '1234',
      }));
      assert.equal(null, getUrl({
        provider: 'Spotify',
        identifier: '1234',
      }));
    });
  });
  describe('.getOwnerUrl', () => {
    it('should return owner url', () => {
      assert.equal('https://www.youtube.com/channel/abcdefg', getOwnerUrl({
        provider: 'YouTube',
        owner_id: 'abcdefg',
      }));
      assert.equal(null, getOwnerUrl({
        provider: 'SoundCloud',
        owner_id: 'abcdefg',
      }));
      assert.equal(null, getOwnerUrl({
        provider: 'Spotify',
        owner_id: 'abcdefg',
      }));
    });
  });
});
