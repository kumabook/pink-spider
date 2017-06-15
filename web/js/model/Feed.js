import ActionList from 'material-ui/svg-icons/action/list';

export const schema = {
  title: 'Feed',
  type:  'object',

  properties: {
    id:           { type: 'string' },
    url:          { type: 'string' },
    title:        { type: 'string' },
    description:  { type: 'string' },
    language:     { type: 'string' },
    velocity:     { type: 'number' },
    state:        { type: 'string' },
    last_updated: { type: 'string' },
    crawled:      { type: 'crawled' },
    visual_url:   { type: 'string', format: 'data-url' },
    icon_url:     { type: 'string', format: 'data-url' },
    cover_url:    { type: 'string', format: 'data-url' },
  },
  required: [],
};

export const tableSchema = {
  'ui:order':   ['visual_url', 'title'],
  'ui:actions': [
    { name: 'entries', icon: ActionList },
  ],
  id:          { 'ui:widget': 'hidden' },
  url:         { 'ui:widget': 'hidden' },
  title:       {},
  description: { 'ui:widget': 'hidden' },
  crawled:     { 'ui:widget': 'hidden' },
  visual_url:  { 'ui:widget': 'img' },
  icon_url:    { 'ui:widget': 'hidden' },
  cover_url:   { 'ui:widget': 'hidden' },
};

export const formSchema = {
};
