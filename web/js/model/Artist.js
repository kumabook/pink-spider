import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import { getCountryParam } from './Playlist';

export const dummu = '';

export const getUrl = (artist) => {
  if (!artist || !artist.identifier) {
    return artist.url;
  }
  const id = artist.identifier;
  switch (artist.provider) {
    case 'YouTube':
      return `https://www.youtube.com/channel/${id}`;
    case 'SoundCloud':
      return `https://soundcloud.com/users/${id}`;
    case 'AppleMusic': {
      const country = getCountryParam(artist.url);
      const n = artist.name;
      return `https://geo.itunes.apple.com/${country}/artist/${n}/id${id}?mt=1&app=music`;
    }
    case 'Spotify':
      return formatOpenURL(parse(artist.url));
    default:
      return artist.url;
  }
};
