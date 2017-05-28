import ActionUpdate from 'material-ui/svg-icons/action/update';
import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import { getCountryParam } from './Playlist';

export const schema = {
  title: 'Artist',
  type:  'object',

  properties: {
    id:            { type: 'string' },
    provider:      { type: 'string' },
    identifier:    { type: 'string' },
    url:           { type: 'string' },
    name:          { type: 'string' },
    thumbnail_url: { type: 'string', format: 'data-url' },
    artwork_url:   { type: 'string', format: 'data-url' },
  },
  required: [],
};

export const tableSchema = {
  'ui:order':   ['thumbnail_url', 'name'],
  'ui:actions': [
    { name: 'reload', icon: ActionUpdate },
  ],
  id:            { 'ui:widget': 'hidden' },
  provider:      { 'ui:widget': 'hidden' },
  identifier:    { 'ui:widget': 'hidden' },
  url:           { 'ui:widget': 'hidden' },
  name:          {},
  thumbnail_url: { 'ui:widget': 'img' },
  artwork_url:   { 'ui:widget': 'hidden' },
};

export const formSchema = {
};

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
