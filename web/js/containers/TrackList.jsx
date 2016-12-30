import React from 'react';
import { connect } from 'react-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import { fetchTracks } from '../actions';

class TrackList extends React.Component {
  static get propTypes() {
    return {
      tracks: React.PropTypes.object.isRequired,
      refresh: React.PropTypes.func,
    };
  }
  componentWillMount() {
    this.props.refresh();
  }
  render() {
    const rows = this.props.tracks.items.map(track => (
      <TableRow key={track.id}>
        <TableRowColumn>
          {track.id}
        </TableRowColumn>
        <TableRowColumn>
          <a href={track.url}>{track.url}</a>
        </TableRowColumn>
        <TableRowColumn>
          {track.title}
        </TableRowColumn>
      </TableRow>
    ));
    return (
      <div>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHeaderColumn>ID</TableHeaderColumn>
              <TableHeaderColumn>url</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows}
          </TableBody>
        </Table>
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    tracks: state.tracks,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    refresh: () => { dispatch(fetchTracks()); },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackList);
