import axios from 'axios';
import { DEFAULT_PAGE, DEFAULT_PER_PAGE } from './pagination';

export default {
  show:  id => axios.get(`/v1/tracks/${id}`).then(response => response.data),
  index: (page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) => axios.get('/v1/tracks', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
  indexByEntry: (entryId, page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) =>
    axios.get(`/v1/entries/${entryId}/tracks`, {
      params: {
        page,
        per_page: perPage,
      },
    }).then(response => response.data),
  update: trackId => axios.post(`/v1/tracks/${trackId}`),
};
