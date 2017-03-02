import axios from 'axios';
import { DEFAULT_PAGE, DEFAULT_PER_PAGE } from './pagination';

export default {
  index: (page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) => axios.get('/v1/entries', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
};
