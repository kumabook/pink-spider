import React        from 'react';
import PropTypes    from 'prop-types';
import {
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import datePrettify from '../utils/datePrettify';
import { getUrl }   from '../model/Album';
import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

function getThumbnailUrl(album) {
  if (album.state === 'alive') {
    return album.thumbnail_url || NO_IMAGE;
  }
  return DEAD_IMAGE;
}

const AlbumListTableRow = ({ album, onDetailButtonClick }) => (
  <TableRow key={album.id}>
    <TableRowColumn>
      <a href={getUrl(album.url)}>
        <img
          src={getThumbnailUrl(album)}
          className="album-list-thumb"
          alt="thumbnail"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      {album.title || `${album.provider} id: ${album.identifier}`}
    </TableRowColumn>
    <TableRowColumn>
      <img
        src={getImageOfProvider(album.provider)}
        width="16"
        height="16"
        alt="provider"
      />
      {album.owner_name}
    </TableRowColumn>
    <TableRowColumn>
      {datePrettify(album.published_at)}
    </TableRowColumn>
    <TableRowColumn>
      <RaisedButton
        label="Detail"
        primary
        onClick={() => onDetailButtonClick(album)}
      />
    </TableRowColumn>
  </TableRow>
);

AlbumListTableRow.propTypes = {
  album: PropTypes.shape({
    title:         PropTypes.string.isRequired,
    owner_name:    PropTypes.string,
    provider:      PropTypes.string.isRequired,
    identifier:    PropTypes.string.isRequired,
    thumbnail_url: PropTypes.string,
  }).isRequired,
  onDetailButtonClick: PropTypes.func.isRequired,
};

AlbumListTableRow.defaultProps = {
  owner_name:    '',
  thumbnail_url: null,
};

export default AlbumListTableRow;
