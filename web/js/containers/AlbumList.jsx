import React              from 'react';
import PropTypes          from 'prop-types';
import { connect }        from 'react-redux';
import { withRouter }     from 'react-router-dom';
import { push }           from 'react-router-redux';
import { Table }          from 'material-jsonschema';
import parseIntOr         from '../utils/parseIntOr';
import { creators }       from '../actions/album';
import { getQuery }       from '../utils/url';
import { defaultPerPage } from '../config';
import {
  schema,
  tableSchema,
  formSchema,
} from '../model/Album';

const { update } = creators;

class AlbumList extends React.Component {
  static get propTypes() {
    return {
      items:   PropTypes.array.isRequired,
      total:   PropTypes.number.isRequired,
      page:    PropTypes.number.isRequired,
      perPage: PropTypes.number.isRequired,
      index:   PropTypes.func.isRequired,
      update:  PropTypes.func.isRequired,
      show:    PropTypes.func.isRequired,
    };
  }
  handleAction(name, item) {
    switch (name) {
      case 'detail':
        this.props.show(item);
        break;
      case 'reload':
        this.props.update(item);
        break;
      default:
        break;
    }
  }
  render() {
    return (
      <Table
        schema={schema}
        tableSchema={tableSchema}
        formSchema={formSchema}
        items={this.props.items}
        page={this.props.page}
        perPage={this.props.perPage}
        pageCount={this.props.total / 10}
        onPageChange={this.props.index}
        canCreate={false}
        canEdit={false}
        canDestroy={false}
        onAction={(name, item) => this.handleAction(name, item)}
      />
    );
  }
}

function mapStateToProps(state) {
  const query   = new URLSearchParams(state.router.location.search);
  const page    = parseIntOr(query.get('page'), 0);
  const perPage = parseIntOr(query.get('per_page'), defaultPerPage);
  return {
    item:  state.albums.item,
    items: state.albums.items,
    total: state.albums.total,
    page,
    perPage,
  };
}

function mapDispatchToProps(dispatch, { match: { params } }) {
  return {
    index: (page, perPage) => {
      const searchParams = new URLSearchParams();
      searchParams.append('page', page);
      searchParams.append('per_page', perPage);
      searchParams.append('query', getQuery(location.search) || '');
      let path = '/albums';
      if (params.entry_id) {
        path = `/entries/${params.entry_id}/albums`;
      }
      dispatch(push({
        pathname: path,
        search:   searchParams.toString(),
      }));
    },
    show:   ({ id }) => dispatch(push({ pathname: `/albums/${id}` })),
    update: item => dispatch(update.start(item)),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(AlbumList));
