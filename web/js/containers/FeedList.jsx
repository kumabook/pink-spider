import React              from 'react';
import PropTypes          from 'prop-types';
import { connect }        from 'react-redux';
import { withRouter }     from 'react-router-dom';
import { push }           from 'react-router-redux';
import { Table }          from 'material-jsonschema';
import parseIntOr         from '../utils/parseIntOr';
import { creators }       from '../actions/feed';
import { defaultPerPage } from '../config';
import {
  schema,
  tableSchema,
  formSchema,
} from '../model/Feed';

const { update } = creators;

class FeedList extends React.Component {
  static get propTypes() {
    return {
      items:         PropTypes.array.isRequired,
      total:         PropTypes.number.isRequired,
      page:          PropTypes.number.isRequired,
      perPage:       PropTypes.number.isRequired,
      index:         PropTypes.func.isRequired,
      update:        PropTypes.func.isRequired,
      entriesOfFeed: PropTypes.func.isRequired,
    };
  }
  handleAction(name, item) {
    switch (name) {
      case 'crawl':
        this.props.update(item);
        break;
      case 'entries':
        this.props.entriesOfFeed(item.id);
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
    item:  state.feeds.item,
    items: state.feeds.items,
    total: state.feeds.total,
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
      dispatch(push({
        pathname: '/feeds',
        search:   params.toString(),
      }));
    },
    entriesOfFeed: (feedId) => {
      const params = new URLSearchParams();
      params.append('page', 0);
      params.append('per_page', defaultPerPage);
      dispatch(push({
        pathname: `/feeds/${feedId}/entries`,
        search:   params.toString(),
      }));
    },
    update: item => dispatch(update.start(item)),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(FeedList));
