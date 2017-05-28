import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  index: (page = 0, perPage = defaultPerPage) => axios.get('/v1/entries', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
};
