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
import { fetchEntries } from '../actions';
import { Status } from '../reducers/entries';
import parseIntOr from '../utils/parseIntOr';

import { DEFAULT_PER_PAGE } from '../api/pagination';

class EntryList extends React.Component {
  static get propTypes() {
    return {
      entries: React.PropTypes.object.isRequired,
      page: React.PropTypes.number,
      fetchEntries: React.PropTypes.func,
      handleEntryClick: React.PropTypes.func,
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
          <a
            href={`/web/entries/${entry.id}/tracks`}
            onClick={this.props.handleEntryClick}
          >
            <img id={entry.id} src={entry.visual_url} className="entry-list-thumb" />
          </a>
        </TableRowColumn>
        <TableRowColumn>
          <a href={entry.url}>{entry.url}</a>
        </TableRowColumn>
        <TableRowColumn>
          {entry.title}
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
    entries: state.entries,
    page: parseIntOr(ownProps.location.query.page, 0),
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  return {
    fetchEntries: (page, perPage) => dispatch(fetchEntries(page, perPage)),
    handleEntryClick: (e) => {
      e.preventDefault();
      const id = e.target.id;
      const location = { pathname: `entries/${id}/tracks` };
      dispatch(push(location));
    },
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
