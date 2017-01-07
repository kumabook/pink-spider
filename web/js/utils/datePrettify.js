import moment from 'moment';

export default dateString => moment(dateString).format('YYYY-MM-DD hh:mm:ss');
