import React        from 'react';
import {
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import datePrettify from '../utils/datePrettify';
import { getUrl }   from '../model/Track';

const NO_IMAGE   = '/web/no_image.png';
const DEAD_IMAGE = '/web/dead_image.png';

function getThumbnailUrl(playlist) {
  if (playlist.state === 'alive') {
    return playlist.thumbnail_url || NO_IMAGE;
  }
  return DEAD_IMAGE;
}

const PlaylistListTableRow = ({ playlist, onDetailButtonClick }) => (
  <TableRow>
    <TableRowColumn>
      <a href={getUrl(playlist.url)}>
        <img
          src={getThumbnailUrl(playlist)}
          role="presentation"
          className="track-list-thumb"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
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
