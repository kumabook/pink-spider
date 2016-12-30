import axios from 'axios';

export default {
  index: () => axios.get('/tracks').then(response => response.data),
};
