import React              from 'react';
import PropTypes          from 'prop-types';
import { connect }        from 'react-redux';
import { withRouter }     from 'react-router-dom';
import { push }           from 'react-router-redux';
import { Table }          from 'material-jsonschema';
import parseIntOr         from '../utils/parseIntOr';
import { getQuery }       from '../utils/url';
import { creators }       from '../actions/artist';
import { defaultPerPage } from '../config';
import {
  schema,
  tableSchema,
  formSchema,
} from '../model/Artist';

const { update } = creators;

class ArtistList extends React.Component {
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
    item:  state.artists.item,
    items: state.artists.items,
    total: state.artists.total,
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
      params.append('query', getQuery(location.search) || '');
      dispatch(push({
        pathname: '/artists',
        search:   params.toString(),
      }));
    },
    show:   ({ id }) => dispatch(push({ pathname: `/artists/${id}` })),
    update: item => dispatch(update.start(item)),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(ArtistList));
