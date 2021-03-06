import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/entries/${id}`).then(r => r.data),
  index: (page = 0, perPage = defaultPerPage, query, feedId) => {
    let path = '/v1/entries';
    if (feedId) {
      path = `/v1/feeds/${feedId}/entries`;
    }
    return axios.get(path, {
      params: {
        page,
        per_page: perPage,
        query,
      },
    }).then(response => response.data);
  },
  update: entry => axios.post(`/v1/entries/${entry.id}`).then(r => r.data),
};
