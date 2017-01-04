import React from 'react';
import { connect } from 'react-redux';
import { push, replace } from 'react-router-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import RaisedButton from 'material-ui/RaisedButton';
import ReactPaginate from 'react-paginate';
import {
  fetchTracks,
  updateTrack,
} from '../actions';
import { Status } from '../reducers/tracks';
import parseIntOr from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE } from '../api/pagination';

const NO_IMAGE   = '/web/no_image.png';
const DEAD_IMAGE = '/web/dead_image.png';

class TrackList extends React.Component {
  static get propTypes() {
    return {
      tracks: React.PropTypes.object.isRequired,
      page: React.PropTypes.number,
      fetchTracks: React.PropTypes.func,
      handleClick: React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
    };
  }
  static getThumbnailUrl(track) {
    if (track.state === 'alive') {
      return track.thumbnail_url || NO_IMAGE;
    }
    return DEAD_IMAGE;
  }
  componentDidUpdate() {
    if (this.props.tracks.status === Status.Dirty) {
      this.props.fetchTracks(this.props.tracks.page,
                             this.props.tracks.perPage);
    }
  }
  render() {
    const rows = this.props.tracks.items.map(track => (
      <TableRow key={track.id}>
        <TableRowColumn>
          <a href={track.url}>
            <img
              src={TrackList.getThumbnailUrl(track)}
              role="presentation"
              className="track-list-thumb"
            />
          </a>
        </TableRowColumn>
        <TableRowColumn>
          {track.title || `${track.provider} id: ${track.identifier}`}
        </TableRowColumn>
        <TableRowColumn>
          {track.description}
        </TableRowColumn>
        <TableRowColumn>
          {track.artist}
        </TableRowColumn>
        <TableRowColumn>
          <RaisedButton label="Update" primary onClick={() => this.props.handleClick(track)} />
        </TableRowColumn>
      </TableRow>
    ));
    const pageCount = this.props.tracks.total / this.props.tracks.perPage;
    const breakLabel = <a href="">...</a>;
    return (
      <div>
        <Table>
          <TableHeader>
            <TableRow>
              <TableHeaderColumn colSpan="3" style={{ textAlign: 'center' }}>
                <ReactPaginate
                  initialPage={this.props.page}
                  previousLabel={'previous'}
                  nextLabel={'next'}
                  breakLabel={breakLabel}
                  breakClassName={'break-me'}
                  pageCount={pageCount}
                  marginPagesDisplayed={2}
                  pageRangeDisplayed={5}
                  containerClassName={'pagination'}
                  subContainerClassName={'pages pagination'}
                  activeClassName={'active'}
                  onPageChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>description</TableHeaderColumn>
              <TableHeaderColumn>artist</TableHeaderColumn>
              <TableHeaderColumn>button</TableHeaderColumn>
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

function mapStateToProps(state, ownProps) {
  return {
    tracks: state.tracks,
    page: parseIntOr(ownProps.location.query.page, 0),
    entry_id: ownProps.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const page = parseIntOr(ownProps.location.query.page, 0);
  const perPage = parseIntOr(ownProps.location.query.per_page, DEFAULT_PER_PAGE);
  const entryId = ownProps.params.entry_id;
  return {
    fetchTracks: () => dispatch(fetchTracks(page, undefined, entryId)),
    handleClick: track => dispatch(updateTrack(track.id)),
    handlePageChange: (data) => {
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
