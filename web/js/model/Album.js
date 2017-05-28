import ContentLink from 'material-ui/svg-icons/content/link';
import ActionUpdate from 'material-ui/svg-icons/action/update';
import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import { getCountryParam } from './Playlist';
import Owner               from '../components/Owner';

export const schema = {
  title: 'Album',
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
  artwork_url:   { 'ui:widget': 'hidden' },
  published_at:  {},
  state:         { 'ui:widget': 'hidden' },
};

export const formSchema = {
};

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
