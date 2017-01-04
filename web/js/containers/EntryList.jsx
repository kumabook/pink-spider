import React from 'react';
import { connect } from 'react-redux';
import { Link } from 'react-router';
import { push, replace } from 'react-router-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import Dialog from 'material-ui/Dialog';
import CircularProgress from 'material-ui/CircularProgress';
import ReactPaginate from 'react-paginate';
import { fetchEntries } from '../actions';
import { Status } from '../reducers/entries';
import parseIntOr from '../utils/parseIntOr';

import { DEFAULT_PER_PAGE } from '../api/pagination';

const NO_IMAGE = '/web/no_image.png';

class EntryList extends React.Component {
  static get propTypes() {
    return {
      entries: React.PropTypes.object.isRequired,
      page: React.PropTypes.number,
      fetchEntries: React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
    };
  }
  componentDidUpdate() {
    if (this.props.entries.status === Status.Dirty) {
      this.props.fetchEntries(this.props.entries.page,
                              this.props.entries.perPage);
    }
  }
  render() {
    const rows = this.props.entries.items.map(entry => (
      <TableRow key={entry.id}>
        <TableRowColumn>
          <a href={entry.url}>
            <img
              src={entry.visual_url || NO_IMAGE}
              role="presentation"
              className="entry-list-thumb"
            />
          </a>
        </TableRowColumn>
        <TableRowColumn>
          {entry.title || `No title: ${entry.url}`}
        </TableRowColumn>
        <TableRowColumn>
          {entry.description}
        </TableRowColumn>
        <TableRowColumn>
          <Link to={`entries/${entry.id}/tracks`}>
            {`${entry.tracks.length} tracks`}
          </Link>
        </TableRowColumn>
      </TableRow>
    ));
    const pageCount = this.props.entries.total / this.props.entries.perPage;
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
              <TableHeaderColumn>tracks</TableHeaderColumn>
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
          open={this.props.entries.status === Status.Dirty}
        >
          <CircularProgress mode="indeterminate" />
        </Dialog>
      </div>
    );
  }
}

function mapStateToProps(state, ownProps) {
  return {
    entries: state.entries,
    page: parseIntOr(ownProps.location.query.page, 0),
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  return {
    fetchEntries: (page, perPage) => dispatch(fetchEntries(page, perPage)),
    handlePageChange: (data) => {
      const perPage = parseIntOr(ownProps.location.query.per_page, DEFAULT_PER_PAGE);
      const location = { pathname: 'entries', query: { page: data.selected, per_page: perPage } };
      if (parseIntOr(ownProps.location.query.page, 0) === data.selected) {
        dispatch(replace(location));
      } else {
        dispatch(push(location));
      }
    },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(EntryList);
