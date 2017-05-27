import React        from 'react';
import PropTypes    from 'prop-types';
import {
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import datePrettify from '../utils/datePrettify';
import { getUrl }   from '../model/Track';
import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

function getThumbnailUrl(track) {
  if (track.state === 'alive') {
    return track.thumbnail_url || NO_IMAGE;
  }
  return DEAD_IMAGE;
}

const TrackListTableRow = ({ track, onDetailButtonClick, onUpdateButtonClick }) => (
  <TableRow key={track.id}>
    <TableRowColumn>
      <a href={getUrl(track.url)}>
        <img
          src={getThumbnailUrl(track)}
          className="track-list-thumb"
          alt="thumbnail"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      {track.title || `${track.provider} id: ${track.identifier}`}
    </TableRowColumn>
    <TableRowColumn>
      <img
        src={getImageOfProvider(track.provider)}
        width="16"
        height="16"
        alt="provider"
      />
      {track.owner_name}
    </TableRowColumn>
    <TableRowColumn>
      {datePrettify(track.published_at)}
    </TableRowColumn>
    <TableRowColumn>
      <RaisedButton
        label="Detail"
        primary
        onClick={() => onDetailButtonClick(track)}
      />
      <br />
      <br />
      <RaisedButton
        label="Update"
        primary
        onClick={() => onUpdateButtonClick(track)}
      />
    </TableRowColumn>
  </TableRow>
);

TrackListTableRow.propTypes = {
  track: PropTypes.shape({
    title:         PropTypes.string.isRequired,
    owner_name:    PropTypes.string,
    provider:      PropTypes.string.isRequired,
    identifier:    PropTypes.string.isRequired,
    thumbnail_url: PropTypes.string,
  }).isRequired,
  onDetailButtonClick: PropTypes.func.isRequired,
  onUpdateButtonClick: PropTypes.func.isRequired,
};

export default TrackListTableRow;
