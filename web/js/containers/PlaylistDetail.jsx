import React from 'react';
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
import { connect }        from 'react-redux';
import { Status }         from '../reducers/playlist';
import { fetchPlaylist }  from '../actions';
import tryGet             from '../utils/tryGet';
import datePrettify       from '../utils/datePrettify';
import { getUrl }         from '../model/Playlist';
import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class PlaylistDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    React.PropTypes.object.isRequired,
      status:                  React.PropTypes.string.isRequired,
      fetchPlaylistIfNeeded:   React.PropTypes.func,
      handleUpdateButtonClick: React.PropTypes.func,
    };
  }
  componentDidMount() {
    this.props.fetchPlaylistIfNeeded(this.props.status);
  }
  componentDidUpdate() {
    this.props.fetchPlaylistIfNeeded(this.props.status);
  }
  render() {
    const id          = tryGet(this.props.item, 'id', 'unknown id');
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const title       = tryGet(this.props.item, 'title', 'No Title');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const identifier  = tryGet(this.props.item, 'identifier', 'No ID');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
    const publishedAt = datePrettify(tryGet(this.props.item, 'published_at', null));
    const createdAt   = datePrettify(tryGet(this.props.item, 'created_at', null));
    const updatedAt   = datePrettify(tryGet(this.props.item, 'updated_at', null));
    const url         = getUrl(this.props.item);
    const overlay = (
      <CardTitle
        title={title}
        subtitle={description}
      />
    );
    const style = {
      margin: 'auto',
      width:  'calc(75vh)',
    };
    return (
      <Card>
        <CardHeader
          title={title}
          subtitle={this.props.item.url}
          avatar={getImageOfProvider(provider)}
        />
        <CardMedia style={style} overlay={overlay} >
          <img role="presentation" src={state === 'alive' ? artworkUrl : DEAD_IMAGE} />
        </CardMedia>
        <CardTitle title={title} />
        <CardActions>
          <RaisedButton
            primary
            label={`View on ${provider}`}
            href={url}
          />
          <RaisedButton
            primary
            label="Update"
            onClick={() => this.props.handleUpdateButtonClick(this.props.item)}
          />
        </CardActions>
        <CardText>
          <List>
            <ListItem primaryText="id" secondaryText={id} />
            <ListItem primaryText="title" secondaryText={title} />
            <ListItem primaryText="state" secondaryText={state} />
            <ListItem primaryText="provider" secondaryText={provider} />
            <ListItem primaryText="identifier" secondaryText={identifier} />
            <ListItem primaryText="published" secondaryText={publishedAt} />
            <ListItem primaryText="created" secondaryText={createdAt} />
            <ListItem primaryText="updated" secondaryText={updatedAt} />
          </List>
        </CardText>
      </Card>
    );
  }
}

function mapStateToProps(state) {
  return {
    ...state.playlist,
  };
}

function mapDispatchToProps(dispatch, ownProps) {
  const playlistId = ownProps.params.playlist_id;
  return {
    fetchPlaylistIfNeeded: (status) => {
      if (status === Status.Dirty) {
        dispatch(fetchPlaylist(playlistId));
      }
    },
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(PlaylistDetail);
