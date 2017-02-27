import url from 'url';
import {
  formatOpenURL,
  parse,
} from 'spotify-uri';

export const getCountryParam = (urlString) => {
  const parsed = url.parse(urlString);
  if (parsed.query && parsed.query.country) {
    return parsed.query.country;
  }
  return 'us';
};

export const getUrl = (playlist) => {
  if (!playlist || !playlist.identifier) {
    return null;
  }
  const id = playlist.identifier;
  switch (playlist.provider) {
    case 'YouTube':
      return `https://www.youtube.com/playlist?list=${id}`;
    case 'SoundCloud':
      return `https://soundcloud.com/playlists/${id}`;
    case 'Spotify':
      return formatOpenURL(parse(playlist.url));
    case 'AppleMusic': {
      const country = getCountryParam(playlist.url);
      return `http://tools.applemusic.com/embed/v1/playlist/pl.${id}?country=${country}`;
    }
    default:
      return playlist.url;
  }
};
