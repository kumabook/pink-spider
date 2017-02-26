import React             from 'react';
import { connect }       from 'react-redux';
import { push, replace } from 'react-router-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
} from 'material-ui/Table';
import Dialog                       from 'material-ui/Dialog';
import CircularProgress             from 'material-ui/CircularProgress';
import { fetchTracks, updateTrack } from '../actions';
import { Status }                   from '../reducers/tracks';
import parseIntOr                   from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE }         from '../api/pagination';
import TrackListTableRow            from '../components/TrackListTableRow';
import Paginate                     from '../components/Paginate';

class TrackList extends React.Component {
  static get propTypes() {
    return {
      tracks:                  React.PropTypes.object.isRequired,
      page:                    React.PropTypes.number,
      fetchTracks:             React.PropTypes.func,
      handleDetailButtonClick: React.PropTypes.func,
      handleUpdateButtonClick: React.PropTypes.func,
      handlePageChange:        React.PropTypes.func,
    };
  }
  componentDidUpdate() {
    if (this.props.tracks.status === Status.Dirty) {
      this.props.fetchTracks(this.props.tracks.page,
                             this.props.tracks.perPage);
    }
  }
  render() {
    const rows = this.props.tracks.items.map(track => (
      <TrackListTableRow
        key={track.id}
        track={track}
        onDetailButtonClick={this.props.handleDetailButtonClick}
        onUpdateButtonClick={this.props.handleUpdateButtonClick}
      />
    ));
    return (
      <div>
        <Table selectable={false}>
          <TableHeader>
            <TableRow>
              <TableHeaderColumn colSpan="5" style={{ textAlign: 'center' }}>
                <Paginate
                  page={this.props.page}
                  pageCount={this.props.tracks.total / this.props.tracks.perPage}
                  onChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>artist</TableHeaderColumn>
              <TableHeaderColumn>published at</TableHeaderColumn>
              <TableHeaderColumn>buttons</TableHeaderColumn>
            </TableRow>
          </TableHeader>
          <TableBody>
            {rows}
          </TableBody>
        </Table>
        <Dialog
          modal
          title="Loading..."
          titleStyle={{ textAlign: 'center' }}
          bodyStyle={{ textAlign: 'center' }}
          open={this.props.tracks.status === Status.Dirty}
        >
          <CircularProgress mode="indeterminate" />
        </Dialog>
      </div>
    );
  }
}

function mapStateToProps(state, ownProps) {
  return {
    tracks:   state.tracks,
    page:     parseIntOr(ownProps.location.query.page, 0),
    entry_id: ownProps.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const page = parseIntOr(ownProps.location.query.page, 0);
  const perPage = parseIntOr(ownProps.location.query.per_page, DEFAULT_PER_PAGE);
  const entryId = ownProps.params.entry_id;
  return {
    fetchTracks:             () => dispatch(fetchTracks(page, undefined, entryId)),
    handleDetailButtonClick: track => dispatch(push({ pathname: `tracks/${track.id}` })),
    handleUpdateButtonClick: track => dispatch(updateTrack(track.id)),
    handlePageChange:        (data) => {
      const path = entryId ? `entries/${entryId}/tracks` : 'tracks';
      const location = { pathname: path, query: { page: data.selected, per_page: perPage } };
      if (parseIntOr(ownProps.location.query.page, 0) === data.selected) {
        dispatch(replace(location));
      } else {
        dispatch(push(location));
      }
    },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackList);
