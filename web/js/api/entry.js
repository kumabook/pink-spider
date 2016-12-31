import axios from 'axios';

export default {
  index: (page = 0, perPage = 10) => axios.get('/entries', {
    params: {
      page,
      per_page: perPage,
    },
  }).then(response => response.data),
};
