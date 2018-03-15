import React              from 'react';
import PropTypes          from 'prop-types';
import { connect }        from 'react-redux';
import { withRouter }     from 'react-router-dom';
import { push }           from 'react-router-redux';
import { Table }          from 'material-jsonschema';
import parseIntOr         from '../utils/parseIntOr';
import { getQuery }       from '../utils/url';
import { creators }       from '../actions/entry';
import { defaultPerPage } from '../config';
import {
  schema,
  tableSchema,
  formSchema,
} from '../model/Entry';

const { update } = creators;

class EntryList extends React.Component {
  static get propTypes() {
    return {
      items:        PropTypes.array.isRequired,
      total:        PropTypes.number.isRequired,
      page:         PropTypes.number.isRequired,
      perPage:      PropTypes.number.isRequired,
      index:        PropTypes.func.isRequired,
      show:         PropTypes.func.isRequired,
      update:       PropTypes.func.isRequired,
      itemsOfEntry: PropTypes.func.isRequired,
    };
  }
  handleAction(name, item) {
    switch (name) {
      case 'reload':
        this.props.update(item);
        break;
      case 'detail':
        this.props.show(item);
        break;
      case 'tracks':
        this.props.itemsOfEntry('tracks', item.id);
        break;
      case 'albums':
        this.props.itemsOfEntry('albums', item.id);
        break;
      case 'playlists':
        this.props.itemsOfEntry('playlists', item.id);
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
  const search  = state.router.location ? state.router.location.search : '';
  const query   = new URLSearchParams(search);
  const page    = parseIntOr(query.get('page'), 0);
  const perPage = parseIntOr(query.get('per_page'), defaultPerPage);
  return {
    item:  state.entries.item,
    items: state.entries.items,
    total: state.entries.total,
    page,
    perPage,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    index: (page, perPage) => {
      const params = new URLSearchParams();
      params.append('page', page);
      params.append('per_page', perPage);
      params.append('query', getQuery(window.location.search) || '');
      dispatch(push({ search: params.toString() }));
    },
    itemsOfEntry: (resourceName, entryId) => {
      const params = new URLSearchParams();
      params.append('page', 0);
      params.append('per_page', defaultPerPage);
      dispatch(push({
        pathname: `/entries/${entryId}/${resourceName}`,
        search:   params.toString(),
      }));
    },
    show:   ({ id }) => dispatch(push({ pathname: `/entries/${id}` })),
    update: item => dispatch(update.start(item)),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(EntryList));
