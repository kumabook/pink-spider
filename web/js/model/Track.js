import {
  formatOpenURL,
  parse,
} from 'spotify-uri';

export const getOwnerUrl = (track) => {
  if (!track || !track.owner_id) {
    return null;
  }
  const ownerId = track.owner_id;
  switch (track.provider) {
    case 'YouTube':
      return `https://www.youtube.com/channel/${ownerId}`;
    case 'SoundCloud':
      return null; // Not support
    default:
      return null;
  }
};

export const getUrl = (track) => {
  if (!track || !track.identifier) {
    return track.url;
  }
  const id = track.identifier;
  switch (track.provider) {
    case 'YouTube':
      return `https://www.youtube.com/watch/?v=${id}`;
    case 'SoundCloud':
      return `https://soundcloud.com/tracks/${id}`;
    case 'Spotify':
      return formatOpenURL(parse(track.url));
    default:
      return track.url;
  }
};
