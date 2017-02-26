import axios from 'axios';
import { DEFAULT_PAGE, DEFAULT_PER_PAGE } from './pagination';

export default {
  show:  id => axios.get(`/albums/${id}`).then(response => response.data),
  index: (page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) => axios.get('/albums', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
  indexByEntry: (entryId, page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) =>
    axios.get(`/entries/${entryId}/albums`, {
      params: {
        page,
        per_page: perPage,
      },
    }).then(response => response.data),
  update: albumId => axios.post(`/albums/${albumId}`),
};
