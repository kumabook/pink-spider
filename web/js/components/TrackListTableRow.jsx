import React        from 'react';
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
          role="presentation"
          className="track-list-thumb"
        />
      </a>
    </TableRowColumn>
    <TableRowColumn>
      {track.title || `${track.provider} id: ${track.identifier}`}
    </TableRowColumn>
    <TableRowColumn>
      <img
        role="presentation"
        src={getImageOfProvider(track.provider)}
        width="16" height="16"
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
  track: React.PropTypes.shape({
    title:         React.PropTypes.string.isRequired,
    owner_name:    React.PropTypes.string.isRequired,
    provider:      React.PropTypes.string.isRequired,
    identifier:    React.PropTypes.string.isRequired,
    thumbnail_url: React.PropTypes.string,
  }),
  onDetailButtonClick: React.PropTypes.func,
  onUpdateButtonClick: React.PropTypes.func,
};

export default TrackListTableRow;
