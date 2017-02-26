import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import { getCountryParam } from './Playlist';

export const getOwnerUrl = (album) => {
  if (!album || !album.owner_id) {
    return null;
  }
//  const ownerId = album.owner_id;
  switch (album.provider) {
    case 'Spotify':
      return null;
    case 'AppleMusic':
      return null;
    default:
      return null;
  }
};

export const getUrl = (album) => {
  if (!album || !album.identifier) {
    return album.url;
  }
  const id = album.identifier;
  switch (album.provider) {
    case 'AppleMusic': {
      const country = getCountryParam(album.url);
      return `http://tools.applemusic.com/embed/v1/album/${id}?country=${country}`;
    }
    case 'Spotify':
      return formatOpenURL(parse(album.url));
    default:
      return album.url;
  }
};
