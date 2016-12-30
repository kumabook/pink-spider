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
import { fetchEntries } from '../actions';

class EntryList extends React.Component {
  static get propTypes() {
    return {
      entries: React.PropTypes.object.isRequired,
      refresh: React.PropTypes.func,
    };
  }
  componentWillMount() {
    this.props.refresh();
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
    entries: state.entries,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    refresh: () => { dispatch(fetchEntries()); },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(EntryList);
