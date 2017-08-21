import ContentLink    from 'material-ui/svg-icons/content/link';
import AVLibraryMusic from 'material-ui/svg-icons/av/library-music';
import AVMusicVideo   from 'material-ui/svg-icons/av/music-video';
import AVQueueMusic   from 'material-ui/svg-icons/av/queue-music';

export const schema = {
  title: 'Entry',
  type:  'object',

  properties: {
    id:          { type: 'string' },
    url:         { type: 'string' },
    title:       { type: 'string' },
    description: { type: 'string' },
    visual_url:  { type: 'string', format: 'data-url' },
    locale:      { type: 'string' },
    created_at:  { type: 'string', format: 'date-time' },
    updated_at:  { type: 'string', format: 'date-time' },
  },
  required: [],
};

export const tableSchema = {
  'ui:order':   ['visual_url', 'title', 'created_at'],
  'ui:actions': [
    { name: 'detail', icon: ContentLink },
    { name: 'tracks', icon: AVMusicVideo },
    { name: 'albums', icon: AVLibraryMusic },
    { name: 'playlists', icon: AVQueueMusic },
  ],
  id:          { 'ui:widget': 'hidden' },
  url:         { 'ui:widget': 'hidden' },
  title:       {},
  description: { 'ui:widget': 'hidden' },
  visual_url:  { 'ui:widget': 'img' },
  locale:      { 'ui:widget': 'hidden' },
};

export const formSchema = {
};
