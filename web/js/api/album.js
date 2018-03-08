import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/albums/${id}`).then(response => response.data),
  index: (page = 0, perPage = defaultPerPage, query, entryId) => {
    let path = '/v1/albums';
    if (entryId) {
      path = `/v1/entries/${entryId}/albums`;
    }
    return axios.get(path, {
      params: {
        page,
        per_page: perPage,
        query,
      },
    }).then(response => response.data);
  },
  update: album => axios.post(`/v1/albums/${album.id}`),
};
