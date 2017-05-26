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
  TableRowColumn,
} from 'material-ui/Table';
import Dialog                       from 'material-ui/Dialog';
import CircularProgress             from 'material-ui/CircularProgress';
import { fetchArtists }             from '../actions';
import { Status }                   from '../reducers/artists';
import parseIntOr                   from '../utils/parseIntOr';
import { DEFAULT_PER_PAGE }         from '../api/pagination';
import Paginate                     from '../components/Paginate';
import { getUrl }                   from '../model/Artist';
import {
  NO_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

function getThumbnailUrl(artist) {
  return artist.thumbnail_url || NO_IMAGE;
}

class ArtistList extends React.Component {
  static get propTypes() {
    return {
      artists:          React.PropTypes.object.isRequired,
      page:             React.PropTypes.number.isRequired,
      fetchArtists:     React.PropTypes.func.isRequired,
      handlePageChange: React.PropTypes.func.isRequired,
    };
  }
  componentDidUpdate() {
    if (this.props.artists.status === Status.Dirty) {
      this.props.fetchArtists(this.props.artists.page,
                              this.props.artists.perPage);
    }
  }
  render() {
    const rows = this.props.artists.items.map(artist => (
      <TableRow key={artist.id}>
        <TableRowColumn>
          <a href={getUrl(artist)}>
            <img
              src={getThumbnailUrl(artist)}
              className="artist-list-thumb"
              alt="thumbnail"
            />
          </a>
        </TableRowColumn>
        <TableRowColumn>
          <img
            src={getImageOfProvider(artist.provider)}
            width="16"
            height="16"
            alt="provider"
          />
          {artist.name || `${artist.provider} id: ${artist.identifier}`}
        </TableRowColumn>
        <TableRowColumn>
          {artist.updated_at}
        </TableRowColumn>
      </TableRow>
    ));
    return (
      <div>
        <Table selectable={false}>
          <TableHeader displaySelectAll={false}>
            <TableRow>
              <TableHeaderColumn colSpan="2" style={{ textAlign: 'center' }}>
                <Paginate
                  page={this.props.page}
                  pageCount={this.props.artists.total / this.props.artists.perPage}
                  onChange={this.props.handlePageChange}
                />
              </TableHeaderColumn>
            </TableRow>
            <TableRow>
              <TableHeaderColumn>thumbnail</TableHeaderColumn>
              <TableHeaderColumn>title</TableHeaderColumn>
              <TableHeaderColumn>updated_at</TableHeaderColumn>
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
          open={this.props.artists.status === Status.Dirty}
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
    artists:  state.artists,
    page:     parseIntOr(query.get('page'), 0),
    entry_id: match.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, { location }) {
  const query = new URLSearchParams(location.search);
  const page = parseIntOr(query.get('page'), 0);
  const perPage = parseIntOr(query.get('per_page'), DEFAULT_PER_PAGE);
  return {
    fetchArtists:     () => dispatch(fetchArtists(page, undefined)),
    handlePageChange: (data) => {
      const path = 'artists';
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

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(ArtistList));
