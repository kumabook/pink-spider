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
import ReactPaginate from 'react-paginate';
import { fetchTracks } from '../actions';
import { Status } from '../reducers/tracks';
import parseIntOr from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE } from '../api/pagination';

class TrackList extends React.Component {
  static get propTypes() {
    return {
      tracks: React.PropTypes.object.isRequired,
      page: React.PropTypes.number,
      fetchTracks: React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
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
