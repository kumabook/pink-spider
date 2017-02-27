import React        from 'react';
import {
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import datePrettify from '../utils/datePrettify';
import { getUrl }   from '../model/Playlist';
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
          role="presentation"
          className="playlist-list-thumb"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      <img
        role="presentation"
        src={getImageOfProvider(playlist.provider)}
        width="16" height="16"
      />
      {playlist.title || `${playlist.provider} id: ${playlist.identifier}`}
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
    provider:      React.PropTypes.string.isRequired,
    identifier:    React.PropTypes.string.isRequired,
    thumbnail_url: React.PropTypes.string,
  }),
  onDetailButtonClick: React.PropTypes.func,
};

export default PlaylistListTableRow;
