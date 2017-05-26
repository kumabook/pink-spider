import React             from 'react';
import { connect }       from 'react-redux';
import { withRouter }    from 'react-router-dom';
import { push, replace } from 'react-router-redux';
import {
  Table,
  TableBody,
  TableHeader,
  TableHeaderColumn,
  TableRow,
} from 'material-ui/Table';
import Dialog               from 'material-ui/Dialog';
import CircularProgress     from 'material-ui/CircularProgress';
import { fetchPlaylists }   from '../actions';
import { Status }           from '../reducers/playlists';
import parseIntOr           from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE } from '../api/pagination';
import PlaylistListTableRow from '../components/PlaylistListTableRow';
import Paginate             from '../components/Paginate';

class PlaylistList extends React.Component {
  static get propTypes() {
    return {
      playlists:               React.PropTypes.object.isRequired,
      page:                    React.PropTypes.number.isRequired,
      fetchPlaylists:          React.PropTypes.func.isRequired,
      handleDetailButtonClick: React.PropTypes.func.isRequired,
      handlePageChange:        React.PropTypes.func.isRequired,
    };
  }
  componentDidUpdate() {
    if (this.props.playlists.status === Status.Dirty) {
      this.props.fetchPlaylists(this.props.playlists.page,
                                this.props.playlists.perPage);
    }
  }
  render() {
    const rows = this.props.playlists.items.map(playlist => (
      <PlaylistListTableRow
        key={playlist.id}
        playlist={playlist}
        onDetailButtonClick={this.props.handleDetailButtonClick}
      />
    ));
    return (
      <div>
        <Table selectable={false}>
          <TableHeader displaySelectAll={false}>
            <TableRow>
              <TableHeaderColumn colSpan="4" style={{ textAlign: 'center' }}>
                <Paginate
                  page={this.props.page}
                  pageCount={this.props.playlists.total / this.props.playlists.perPage}
                  onChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>owner</TableHeaderColumn>
              <TableHeaderColumn>published at</TableHeaderColumn>
              <TableHeaderColumn>buttons</TableHeaderColumn>
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
          open={this.props.playlists.status === Status.Dirty}
        >
          <CircularProgress mode="indeterminate" />
        </Dialog>
      </div>
    );
  }
}

function mapStateToProps(state, { location, match }) {
  const query = new URLSearchParams(location.search);
  return {
    playlists: state.playlists,
    page:      parseIntOr(query.get('page'), 0),
    entry_id:  match.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, { location, match }) {
  const query = new URLSearchParams(location.search);
  const page = parseIntOr(query.get('page'), 0);
  const perPage = parseIntOr(query.get('per_page'), DEFAULT_PER_PAGE);
  const entryId = match.params.entry_id;
  return {
    fetchPlaylists:          () => dispatch(fetchPlaylists(page, undefined, entryId)),
    handleDetailButtonClick: playlist => dispatch(push({ pathname: `playlists/${playlist.id}` })),
    handlePageChange:        (data) => {
      const path = entryId ? `entries/${entryId}/playlists` : 'playlists';
      const params = new URLSearchParams();
      params.append('page', data.selected);
      params.append('per_page', perPage);
      const loc = { pathname: path, search: params.toString() };
      if (parseIntOr(query.get('page'), 0) === data.selected) {
        dispatch(replace(loc));
      } else {
        dispatch(push(loc));
      }
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(PlaylistList));
