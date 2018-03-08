import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/tracks/${id}`).then(response => response.data),
  index: (page = 0, perPage = defaultPerPage, query, entryId) => {
    let path = '/v1/tracks';
    if (entryId) {
      path = `/v1/entries/${entryId}/tracks`;
    }
    return axios.get(path, {
      params: {
        page,
        per_page: perPage,
        query,
      },
    }).then(response => response.data);
  },
  update: track => axios.post(`/v1/tracks/${track.id}`),
};
