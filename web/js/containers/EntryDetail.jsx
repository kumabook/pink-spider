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
import RaisedButton     from 'material-ui/RaisedButton';
import FlatButton       from 'material-ui/FlatButton';
import Dialog           from 'material-ui/Dialog';
import { PropertyList } from 'material-jsonschema';
import { connect }      from 'react-redux';
import { creators }     from '../actions/entry';
import { schema }       from '../model/Entry';

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
  previewDialogTitle() {
    switch (this.props.previewType) {
      case 'content': {
        return this.props.item.title;
      }
      case 'text':
        return this.props.item.title;
      default:
        return null;
    }
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
            <PropertyList schema={schema} item={this.props.item} />;
          </CardText>
        </Card>
        <Dialog
          title={this.previewDialogTitle()}
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
