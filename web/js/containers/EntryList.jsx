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
import { fetchEntries } from '../actions';

class EntryList extends React.Component {
  static get propTypes() {
    return {
      entries: React.PropTypes.object.isRequired,
      fetchEntries: React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
    };
  }
  componentWillMount() {
    this.props.fetchEntries(0, 10);
  }
  render() {
    const rows = this.props.entries.items.map(entry => (
      <TableRow key={entry.id}>
        <TableRowColumn>
          {entry.id}
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
    entries: state.entries,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    fetchEntries: (page, perPage) => dispatch(fetchEntries(page, perPage)),
    handlePageChange: data => dispatch(fetchEntries(data.selected)),
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(EntryList);
