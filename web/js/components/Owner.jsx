import React                  from 'react';
import PropTypes              from 'prop-types';
import { getImageOfProvider } from '../utils/thumbnail';

const Owner = ({ item }) => (
  <div>
    <img
      src={getImageOfProvider(item && item.provider)}
      width="16"
      height="16"
      alt="provider"
    />
    {item && (item.owner_name || item.owner_id)}
  </div>
);

Owner.propTypes = {
  item: PropTypes.object, // eslint-disable-line react/forbid-prop-types
};
Owner.defaultProps = {
  item: {},
};

export default Owner;
