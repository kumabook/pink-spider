import ContentLink from 'material-ui/svg-icons/content/link';
import ActionUpdate from 'material-ui/svg-icons/action/update';
import url from 'url';
import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import Owner from '../components/Owner';

export const schema = {
  title: 'Playlist',
  type:  'object',

  properties: {
    id:            { type: 'string' },
    provider:      { type: 'string' },
    identifier:    { type: 'string' },
    owner_id:      { type: 'string' },
    owner_name:    { type: 'string' },
    url:           { type: 'string' },
    title:         { type: 'string' },
    description:   { type: 'string' },
    velocity:      { type: 'number' },
    thumbnail_url: { type: 'string', format: 'data-url' },
    artwork_url:   { type: 'string', format: 'data-url' },
    published_at:  { type: 'string', format: 'date-time' },
    state:         { type: 'string' },
  },
  required: [],
};

export const tableSchema = {
  'ui:order':   ['thumbnail_url', 'title', 'published_at'],
  'ui:actions': [
    { name: 'detail', icon: ContentLink },
    { name: 'reload', icon: ActionUpdate },
  ],
  thumbnail_url: { 'ui:widget': 'img' },
  id:            { 'ui:widget': 'hidden' },
  provider:      { 'ui:widget': 'hidden' },
  identifier:    { 'ui:widget': 'hidden' },
  owner_id:      { 'ui:widget': 'hidden' },
  owner_name:    { 'ui:widget': Owner },
  url:           { 'ui:widget': 'hidden' },
  title:         {},
  description:   { 'ui:widget': 'hidden' },
  velocity:      {},
  artwork_url:   { 'ui:widget': 'hidden' },
  published_at:  {},
  state:         { 'ui:widget': 'hidden' },
};

export const formSchema = {
  id: { 'ui:widget': 'hidden' },
};

export const getCountryParam = (urlString) => {
  const parsed = url.parse(urlString);
  if (parsed.query && parsed.query.country) {
    return parsed.query.country;
  }
  return 'us';
};

export const getOwnerName = playlist => playlist.owner_name || playlist.owner_id;

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
