import { createResourceActions } from 'material-jsonschema';
import { schema }                from '../model/Entry';

const actions = createResourceActions(schema);

export default actions;

export const index   = actions.index;
export const show    = actions.show;
export const create  = actions.create;
export const update  = actions.update;
export const destroy = actions.destroy;

export const creators = actions.creators;

export const preview = 'PREVIEW_ENTRY';
actions.creators.preview = payload => ({ type: preview, payload });
