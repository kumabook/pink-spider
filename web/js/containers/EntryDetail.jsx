import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import { List, ListItem } from 'material-ui/List';
import RaisedButton       from 'material-ui/RaisedButton';
import FlatButton         from 'material-ui/FlatButton';
import Dialog             from 'material-ui/Dialog';
import { connect }        from 'react-redux';
import { creators }       from '../actions/entry';

class EntryDetail extends React.Component {
  static get propTypes() {
    return {
      item:           PropTypes.object.isRequired,
      previewType:    PropTypes.string.isRequired,
      update:         PropTypes.func.isRequired,
      previewContent: PropTypes.func.isRequired,
      previewText:    PropTypes.func.isRequired,
      finishPreview:  PropTypes.func.isRequired,
    };
  }
  /* eslint react/no-danger: 0 */
  previewDialogBody() {
    switch (this.props.previewType) {
      case 'content': {
        const markup = { __html: this.props.item[this.props.previewType] };
        return <div dangerouslySetInnerHTML={markup} />;
      }
      case 'text':
        return <div>{this.props.item[this.props.previewType]}</div>;
      default:
        return null;
    }
  }
  previewDialogIsHidden() {
    return !!this.props.item[this.props.previewType];
  }
  render() {
    const overlay = (
      <CardTitle />
    );
    const style = {
      margin: 'auto',
      width:  'calc(75vh)',
    };
    const actions = [
      <FlatButton
        label="Close"
        primary
        onClick={this.props.finishPreview}
      />,
    ];
    return (
      <div>
        <Card>
          <CardHeader
            title={this.props.item.title}
            subtitle={this.props.item.url}
          />
          <CardMedia style={style} overlay={overlay} >
            <img alt="visual" src={this.props.item.visual_url} />
          </CardMedia>
          <CardTitle title={this.props.item.title} />
          <CardActions>
            <RaisedButton
              primary
              label={'View original'}
              href={this.props.item.url}
            />
            <RaisedButton
              primary
              label={'View content'}
              onClick={() => this.props.previewContent(this.props.item)}
            />
            <RaisedButton
              primary
              label={'View text'}
              onClick={() => this.props.previewText(this.props.item)}
            />
            <RaisedButton
              label={'Update'}
              onClick={() => this.props.update(this.props.item)}
            />
          </CardActions>
          <CardText>
            <List>
              <ListItem primaryText="id" secondaryText={this.props.item.id} />
              <ListItem primaryText="title" secondaryText={this.props.item.title} />
              <ListItem primaryText="description" secondaryText={this.props.item.description} />
              <ListItem primaryText="visual_url" secondaryText={this.props.item.visual_url} />
              <ListItem primaryText="locale" secondaryText={this.props.item.locale} />
              <ListItem primaryText="summary" secondaryText={this.props.item.summary} />
              <ListItem primaryText="author" secondaryText={this.props.item.author} />
              <ListItem primaryText="crawled" secondaryText={this.props.item.crawled} />
              <ListItem primaryText="published" secondaryText={this.props.item.published} />
              <ListItem primaryText="updated" secondaryText={this.props.item.updated} />
              <ListItem primaryText="fingerprint" secondaryText={this.props.item.fingerprint} />
              <ListItem primaryText="origin_id" secondaryText={this.props.item.origin_id} />
              <ListItem primaryText="feed_id" secondaryText={this.props.item.feed_id} />

              <ListItem primaryText="created_at" secondaryText={this.props.item.created_at} />
              <ListItem primaryText="updated_at" secondaryText={this.props.item.updated_at} />
            </List>
          </CardText>
        </Card>
        <Dialog
          title="Preview"
          actions={actions}
          modal={false}
          open={this.previewDialogIsHidden()}
          onRequestClose={this.props.finishPreview}
          autoScrollBodyContent
        >
          {this.previewDialogBody()}
        </Dialog>
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    item:        state.entries.item,
    previewType: state.entries.previewType,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    update:         item => dispatch(creators.update.start(item)),
    previewContent: () => dispatch(creators.preview('content')),
    previewText:    () => dispatch(creators.preview('text')),
    finishPreview:  () => dispatch(creators.preview('hidden')),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(EntryDetail));
