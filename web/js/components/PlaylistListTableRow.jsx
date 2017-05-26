import React        from 'react';
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
  playlist: React.PropTypes.shape({
    title:         React.PropTypes.string.isRequired,
    owner_name:    React.PropTypes.string,
    provider:      React.PropTypes.string.isRequired,
    identifier:    React.PropTypes.string.isRequired,
    thumbnail_url: React.PropTypes.string,
  }).isRequired,
  onDetailButtonClick: React.PropTypes.func.isRequired,
};

export default PlaylistListTableRow;
