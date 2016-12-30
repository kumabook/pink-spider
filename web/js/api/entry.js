import axios from 'axios';

export default {
  index: () => axios.get('/entries').then(response => response.data),
};
