import ContentLink from 'material-ui/svg-icons/content/link';
import ActionUpdate from 'material-ui/svg-icons/action/update';
import {
  formatOpenURL,
  parse,
} from 'spotify-uri';
import Owner from '../components/Owner';

export const schema = {
  title: 'Track',
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
    audio_url:     { type: 'string', format: 'data-url' },
    duration:      { type: 'integer' },
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
  audio_url:     { 'ui:widget': 'hidden' },
  duration:      { 'ui:widget': 'hidden' },
  published_at:  {},
  state:         { 'ui:widget': 'hidden' },
};

export const formSchema = {
};

export const getCountry = (urlString) => {
  const regex = /itunes\.apple\.com\/([a-zA-Z]+)\//;
  const results = urlString.match(regex);
  if (results && results.length >= 2) {
    return results[1];
  }
  return 'us';
};


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
      return track.url;
    case 'Spotify':
      return formatOpenURL(parse(track.url));
    case 'AppleMusic': {
      const country = getCountry(track.url);
      return `http://tools.applemusic.com/embed/v1/song/${id}?country=${country}`;
    }
    default:
      return track.url;
  }
};
