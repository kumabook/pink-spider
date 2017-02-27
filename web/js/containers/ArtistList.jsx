import React             from 'react';
import { connect }       from 'react-redux';
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
      page:             React.PropTypes.number,
      fetchArtists:     React.PropTypes.func,
      handlePageChange: React.PropTypes.func,
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
              role="presentation"
              className="artist-list-thumb"
            />
          </a>
        </TableRowColumn>
        <TableRowColumn>
          <img
            role="presentation"
            src={getImageOfProvider(artist.provider)}
            width="16" height="16"
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

function mapStateToProps(state, ownProps) {
  return {
    artists:  state.artists,
    page:     parseIntOr(ownProps.location.query.page, 0),
    entry_id: ownProps.params.entry_id,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const page = parseIntOr(ownProps.location.query.page, 0);
  const perPage = parseIntOr(ownProps.location.query.per_page, DEFAULT_PER_PAGE);
  return {
    fetchArtists:     () => dispatch(fetchArtists(page, undefined)),
    handlePageChange: (data) => {
      const path = 'artists';
      const location = { pathname: path, query: { page: data.selected, per_page: perPage } };
      if (parseIntOr(ownProps.location.query.page, 0) === data.selected) {
        dispatch(replace(location));
      } else {
        dispatch(push(location));
      }
    },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(ArtistList);
