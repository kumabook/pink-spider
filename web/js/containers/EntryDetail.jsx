import React from 'react';
import PropTypes from 'prop-types';
import { push } from 'react-router-redux';
import { withRouter } from 'react-router-dom';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import ContentLink        from 'material-ui/svg-icons/content/link';
import RaisedButton       from 'material-ui/RaisedButton';
import FlatButton         from 'material-ui/FlatButton';
import Dialog             from 'material-ui/Dialog';
import { Tabs, Tab }      from 'material-ui/Tabs';
import { List, ListItem } from 'material-ui/List';
import { PropertyList }   from 'material-jsonschema';
import { connect }        from 'react-redux';
import { creators }       from '../actions/entry';
import { schema }         from '../model/Entry';

class EntryDetail extends React.Component {
  static get propTypes() {
    return {
      item:                PropTypes.object.isRequired,
      previewType:         PropTypes.string.isRequired,
      update:              PropTypes.func.isRequired,
      previewContent:      PropTypes.func.isRequired,
      previewText:         PropTypes.func.isRequired,
      finishPreview:       PropTypes.func.isRequired,
      handleTrackClick:    PropTypes.func.isRequired,
      handleAlbumClick:    PropTypes.func.isRequired,
      handlePlaylistClick: PropTypes.func.isRequired,
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
  renderSummaryCard() {
    const overlay = (
      <CardTitle />
    );
    const style = {
      margin: 'auto',
      width:  'calc(75vh)',
    };
    return (
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
        <CardText />
      </Card>
    );
  }
  renderPropsList() {
    return <PropertyList schema={schema} item={this.props.item} />;
  }
  renderTrackList() {
    if (!this.props.item.tracks) {
      return null;
    }
    const items = this.props.item.tracks.map(track => (
      <ListItem
        key={track.id}
        primaryText={track.title}
        rightIcon={<ContentLink onClick={() => this.props.handleTrackClick(track)} />}
      />
    ));
    return <List>{items}</List>;
  }
  renderAlbumList() {
    if (!this.props.item.albums) {
      return null;
    }
    const items = this.props.item.albums.map(album => (
      <ListItem
        key={album.id}
        primaryText={album.title}
        rightIcon={<ContentLink onClick={() => this.props.handleAlbumClick(album)} />}
      />
    ));
    return <List>{items}</List>;
  }
  renderPlaylistList() {
    if (!this.props.item.playlists) {
      return null;
    }
    const items = this.props.item.playlists.map(playlist => (
      <ListItem
        key={playlist.id}
        primaryText={playlist.title}
        rightIcon={<ContentLink onClick={() => this.props.handlePlaylistClick(playlist)} />}
      />
    ));
    return <List>{items}</List>;
  }
  render() {
    const actions = [
      <FlatButton
        label="Close"
        primary
        onClick={this.props.finishPreview}
      />,
    ];
    return (
      <div>
        <Tabs>
          <Tab label="Summary">
            {this.renderSummaryCard()}
          </Tab>
          <Tab label="Props">
            {this.renderPropsList()}
          </Tab>
          <Tab label="Tracks" >
            {this.renderTrackList()}
          </Tab>
          <Tab label="Albums" >
            {this.renderAlbumList()}
          </Tab>
          <Tab label="Playlists" >
            {this.renderPlaylistList()}
          </Tab>
        </Tabs>
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
    update:              item => dispatch(creators.update.start(item)),
    previewContent:      () => dispatch(creators.preview('content')),
    previewText:         () => dispatch(creators.preview('text')),
    finishPreview:       () => dispatch(creators.preview('hidden')),
    handleTrackClick:    ({ id }) => dispatch(push({ pathname: `/tracks/${id}` })),
    handleAlbumClick:    ({ id }) => dispatch(push({ pathname: `/albums/${id}` })),
    handlePlaylistClick: ({ id }) => dispatch(push({ pathname: `/playlists/${id}` })),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(EntryDetail));
