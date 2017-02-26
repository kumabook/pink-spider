import React        from 'react';
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
          role="presentation"
          className="album-list-thumb"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      {album.title || `${album.provider} id: ${album.identifier}`}
    </TableRowColumn>
    <TableRowColumn>
      <img
        role="presentation"
        src={getImageOfProvider(album.provider)}
        width="16" height="16"
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
  album: React.PropTypes.shape({
    title:         React.PropTypes.string.isRequired,
    owner_name:    React.PropTypes.string,
    provider:      React.PropTypes.string.isRequired,
    identifier:    React.PropTypes.string.isRequired,
    thumbnail_url: React.PropTypes.string,
  }),
  onDetailButtonClick: React.PropTypes.func,
};

export default AlbumListTableRow;
