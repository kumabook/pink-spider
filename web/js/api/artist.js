import axios from 'axios';
import { DEFAULT_PAGE, DEFAULT_PER_PAGE } from './pagination';

export default {
  show:  id => axios.get(`/artists/${id}`).then(response => response.data),
  index: (page = DEFAULT_PAGE, perPage = DEFAULT_PER_PAGE) => axios.get('/artists', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
  update: artistId => axios.post(`/artists/${artistId}`),
};
