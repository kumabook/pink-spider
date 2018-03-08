import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/feeds/${id}`).then(response => response.data),
  index: (page = 0, perPage = defaultPerPage, query) => axios.get('/v1/feeds', {
    params: {
      page,
      per_page: perPage,
      query,
    },
  }).then(response => response.data),
  update: feed => axios.post(`/v1/feeds/${feed.id}`),
};
