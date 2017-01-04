import axios from 'axios';
import { DEFAULT_PAGE, DEFAULT_PER_PAGE } from './pagination';

export default {
  index: (page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) => axios.get('/tracks', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
  indexByEntry: (entryId, page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) =>
    axios.get(`/entries/${entryId}/tracks`, {
      params: {
        page,
        per_page: perPage,
      },
    }).then(response => response.data),
  update: trackId => axios.post(`/tracks/${trackId}`),
};
