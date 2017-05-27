import React             from 'react';
import PropTypes         from 'prop-types';
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
import Dialog                       from 'material-ui/Dialog';
import CircularProgress             from 'material-ui/CircularProgress';
import { fetchAlbums }              from '../actions';
import { Status }                   from '../reducers/albums';
import parseIntOr                   from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE }         from '../api/pagination';
import AlbumListTableRow            from '../components/AlbumListTableRow';
import Paginate                     from '../components/Paginate';

class AlbumList extends React.Component {
  static get propTypes() {
    return {
      albums:                  PropTypes.object.isRequired,
      page:                    PropTypes.number.isRequired,
      fetchAlbums:             PropTypes.func.isRequired,
      handleDetailButtonClick: PropTypes.func.isRequired,
      handleUpdateButtonClick: PropTypes.func.isRequired,
      handlePageChange:        PropTypes.func.isRequired,
    };
  }
  componentDidUpdate() {
    if (this.props.albums.status === Status.Dirty) {
      this.props.fetchAlbums(this.props.albums.page,
                             this.props.albums.perPage);
    }
  }
  render() {
    const rows = this.props.albums.items.map(album => (
      <AlbumListTableRow
        key={album.id}
        album={album}
        onDetailButtonClick={this.props.handleDetailButtonClick}
        onUpdateButtonClick={this.props.handleUpdateButtonClick}
      />
    ));
    return (
      <div>
        <Table selectable={false}>
          <TableHeader displaySelectAll={false}>
            <TableRow>
              <TableHeaderColumn colSpan="5" style={{ textAlign: 'center' }}>
                <Paginate
                  page={this.props.page}
                  pageCount={this.props.albums.total / this.props.albums.perPage}
                  onChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>artist</TableHeaderColumn>
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
          open={this.props.albums.status === Status.Dirty}
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
    albums:   state.albums,
    page:     parseIntOr(query.get('page'), 0),
    entry_id: match.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, { location, match }) {
  const query = new URLSearchParams(location.search);
  const page = parseIntOr(query.get('page'), 0);
  const perPage = parseIntOr(query.get('per_page'), DEFAULT_PER_PAGE);
  const entryId = match.params.entry_id;
  return {
    fetchAlbums:             () => dispatch(fetchAlbums(page, undefined, entryId)),
    handleDetailButtonClick: album => dispatch(push({ pathname: `albums/${album.id}` })),
    handlePageChange:        (data) => {
      const path = entryId ? `entries/${entryId}/albums` : 'albums';
      const params = new URLSearchParams();
      params.append('page', data.selected);
      params.append('per_page', perPage);
      const loc = { pathname: path, search: params.toString() };
      if (parseIntOr(query.get('page'), 0) === data.seltected) {
        dispatch(replace(loc));
      } else {
        dispatch(push(loc));
      }
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(AlbumList));
