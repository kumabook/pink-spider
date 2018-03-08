/* global URLSearchParams */

import parseIntOr from './parseIntOr';
import { defaultPerPage } from '../config';

export const getSearchParams = s => new URLSearchParams(s);
export const getPage         = s => parseIntOr(getSearchParams(s).get('page'), 0);
export const getPerPage      = s => parseIntOr(getSearchParams(s).get('per_page'), defaultPerPage);
export const getQuery        = s => getSearchParams(s).get('query');
