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
import ReactPaginate from 'react-paginate';
import { fetchTracks } from '../actions';

class TrackList extends React.Component {
  static get propTypes() {
    return {
      tracks: React.PropTypes.object.isRequired,
      fetchTracks: React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
    };
  }
  componentWillMount() {
    this.props.fetchTracks();
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

function mapStateToProps(state) {
  return {
    tracks: state.tracks,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    fetchTracks: () => dispatch(fetchTracks()),
    handlePageChange: data => dispatch(fetchTracks(data.selected)),
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackList);
