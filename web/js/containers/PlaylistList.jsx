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
      page:                    React.PropTypes.number,
      fetchPlaylists:          React.PropTypes.func,
      handleDetailButtonClick: React.PropTypes.func,
      handlePageChange:        React.PropTypes.func,
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

function mapStateToProps(state, ownProps) {
  return {
    playlists: state.playlists,
    page:      parseIntOr(ownProps.location.query.page, 0),
    entry_id:  ownProps.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const page = parseIntOr(ownProps.location.query.page, 0);
  const perPage = parseIntOr(ownProps.location.query.per_page, DEFAULT_PER_PAGE);
  const entryId = ownProps.params.entry_id;
  return {
    fetchPlaylists:          () => dispatch(fetchPlaylists(page, undefined, entryId)),
    handleDetailButtonClick: playlist => dispatch(push({ pathname: `playlists/${playlist.id}` })),
    handlePageChange:        (data) => {
      const path = entryId ? `entries/${entryId}/playlists` : 'playlists';
      const location = { pathname: path, query: { page: data.selected, per_page: perPage } };
      if (parseIntOr(ownProps.location.query.page, 0) === data.selected) {
        dispatch(replace(location));
      } else {
        dispatch(push(location));
      }
    },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(PlaylistList);
