import React                from 'react';
import { connect }          from 'react-redux';
import { withRouter, Link } from 'react-router-dom';
import { push, replace }    from 'react-router-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
  TableRowColumn,
} from 'material-ui/Table';
import Dialog           from 'material-ui/Dialog';
import RaisedButton     from 'material-ui/RaisedButton';
import CircularProgress from 'material-ui/CircularProgress';
import { Status }       from '../reducers/entries';
import parseIntOr       from '../utils/parseIntOr';
import datePrettify     from '../utils/datePrettify';
import Paginate         from '../components/Paginate';
import {
  fetchEntries,
  playlistify,
} from '../actions';

import { DEFAULT_PER_PAGE } from '../api/pagination';
import { NO_IMAGE }         from '../utils/thumbnail';

class EntryList extends React.Component {
  static get propTypes() {
    return {
      entries:             React.PropTypes.object.isRequired,
      page:                React.PropTypes.number,
      fetchEntries:        React.PropTypes.func,
      handlePageChange:    React.PropTypes.func,
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
          {datePrettify(entry.updated_at)}
        </TableRowColumn>
        <TableRowColumn>
          <Link to={`entries/${entry.id}/tracks`}>
            {`${entry.tracks.length} tracks`}
          </Link>
          <br />
          <Link to={`entries/${entry.id}/playlists`}>
            {`${entry.playlists.length} playlists`}
          </Link>
          <br />
          <Link to={`entries/${entry.id}/albums`}>
            {`${entry.albums.length} albums`}
          </Link>
        </TableRowColumn>
      </TableRow>
    ));
    return (
      <div>
        <Table selectable={false}>
          <TableHeader displaySelectAll={false}>
            <TableRow>
              <TableHeaderColumn colSpan="5" style={{ textAlign: 'center' }}>
                <Paginate
                  page={this.props.page}
                  pageCount={this.props.entries.total / this.props.entries.perPage}
                  onChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>description</TableHeaderColumn>
              <TableHeaderColumn>updated at</TableHeaderColumn>
              <TableHeaderColumn>tracks</TableHeaderColumn>
            </TableRow>
          </TableHeader>
          <TableBody displayRowCheckbox={false}>
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

function mapStateToProps(state, { location: { search } }) {
  const query = new URLSearchParams(search);
  return {
    entries: state.entries,
    page:    parseIntOr(query.get('page'), 0),
  };
}

function mapDispatchToProps(dispatch, { location: { search } }) {
  const query = new URLSearchParams(search);
  return {
    fetchEntries:     (page, perPage) => dispatch(fetchEntries(page, perPage)),
    handlePageChange: (data) => {
      const perPage  = parseIntOr(query.get('per_page'), DEFAULT_PER_PAGE);
      const params = new URLSearchParams();
      params.append('page', data.selected);
      params.append('per_page', perPage);
      const location = { pathname: 'entries', search: params.toString() };
      if (parseIntOr(query.get('page'), 0) === data.selected) {
        dispatch(replace(location));
      } else {
        dispatch(push(location));
      }
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(EntryList));
