import React        from 'react';
import PropTypes    from 'prop-types';
import {
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import datePrettify from '../utils/datePrettify';
import {
  getOwnerName,
  getUrl,
} from '../model/Playlist';
import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

function getThumbnailUrl(playlist) {
  if (playlist.state === 'alive') {
    return playlist.thumbnail_url || NO_IMAGE;
  }
  return DEAD_IMAGE;
}

const PlaylistListTableRow = ({ playlist, onDetailButtonClick }) => (
  <TableRow>
    <TableRowColumn>
      <a href={getUrl(playlist)}>
        <img
          src={getThumbnailUrl(playlist)}
          className="playlist-list-thumb"
          alt="thumbnail"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      {playlist.title || `${playlist.provider} id: ${playlist.identifier}`}
    </TableRowColumn>
    <TableRowColumn>
      <img
        src={getImageOfProvider(playlist.provider)}
        width="16"
        height="16"
        alt="provider"
      />
      {getOwnerName(playlist)}
    </TableRowColumn>
    <TableRowColumn>
      {datePrettify(playlist.published_at)}
    </TableRowColumn>
    <TableRowColumn>
      <RaisedButton
        label="Detail"
        primary
        onClick={() => onDetailButtonClick(playlist)}
      />
    </TableRowColumn>
  </TableRow>
);

PlaylistListTableRow.propTypes = {
  playlist: PropTypes.shape({
    title:         PropTypes.string.isRequired,
    owner_name:    PropTypes.string,
    provider:      PropTypes.string.isRequired,
    identifier:    PropTypes.string.isRequired,
    thumbnail_url: PropTypes.string,
  }).isRequired,
  onDetailButtonClick: PropTypes.func.isRequired,
};

export default PlaylistListTableRow;
