import axios              from 'axios';
import { defaultPerPage } from '../config';

export default {
  show:  id => axios.get(`/v1/playlists/${id}`).then(response => response.data),
  index: (page = 0, perPage = defaultPerPage, query, entryId) => {
    let path = '/v1/playlists';
    if (entryId) {
      path = `/v1/entries/${entryId}/playlists`;
    }
    return axios.get(path, {
      params: {
        page,
        per_page: perPage,
        query,
      },
    }).then(response => response.data);
  },
  update: playlist => axios.post(`/v1/playlists/${playlist.id}`, playlist),
};
