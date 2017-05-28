import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/artists/${id}`).then(response => response.data),
  index: (page = 0, perPage = defaultPerPage) => axios.get('/v1/artists', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
  update: artist => axios.post(`/v1/artists/${artist.id}`),
};
