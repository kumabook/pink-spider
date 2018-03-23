import React from 'react';
import PropTypes from 'prop-types';
import { push } from 'react-router-redux';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import { PropertyList }   from 'material-jsonschema';
import { Tabs, Tab }      from 'material-ui/Tabs';
import { List, ListItem } from 'material-ui/List';
import ContentLink        from 'material-ui/svg-icons/content/link';
import RaisedButton       from 'material-ui/RaisedButton';
import { connect }        from 'react-redux';
import { creators }       from '../actions/track';
import tryGet             from '../utils/tryGet';
import {
  getUrl,
  getOwnerUrl,
  schema,
} from '../model/Track';

import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class TrackDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    PropTypes.object.isRequired,
      handleUpdateButtonClick: PropTypes.func.isRequired,
      handleArtistClick:       PropTypes.func.isRequired,
      handlePlaylistClick:     PropTypes.func.isRequired,
    };
  }
  renderSummaryCard() {
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const title       = tryGet(this.props.item, 'title', 'No Title');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
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
    const ownerUrl = getOwnerUrl(this.props.item);
    const ownerLink = <a href={ownerUrl}>{tryGet(this.props.item, 'owner_name', 'Unknown')}</a>;
    return (
      <Card>
        <CardHeader
          title={ownerLink}
          subtitle={tryGet(this.props.item, 'owner_id', 'Unknown')}
          avatar={getImageOfProvider(provider)}
        />
        <CardMedia style={style} overlay={overlay} >
          <img alt="artwork" src={state === 'alive' ? artworkUrl : DEAD_IMAGE} />
        </CardMedia>
        <CardTitle title={title} />
        <CardActions>
          <RaisedButton
            primary
            label={`View on ${provider}`}
            href={getUrl(this.props.item)}
          />
          <RaisedButton
            primary
            label="Update"
            onClick={() => this.props.handleUpdateButtonClick(this.props.item)}
          />
        </CardActions>
        <CardText />
      </Card>
    );
  }
  renderPropsList() {
    return <PropertyList schema={schema} item={this.props.item} />;
  }
  renderArtistList() {
    if (!this.props.item.artists) {
      return null;
    }
    const items = this.props.item.artists.map(artist => (
      <ListItem
        key={artist.id}
        primaryText={artist.name}
        rightIcon={<ContentLink onClick={() => this.props.handleArtistClick(artist)} />}
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
    return (
      <Tabs>
        <Tab label="Summary">
          {this.renderSummaryCard()}
        </Tab>
        <Tab label="Props">
          {this.renderPropsList()}
        </Tab>
        <Tab label="Artists" >
          {this.renderArtistList()}
        </Tab>
        <Tab label="Playlists" >
          {this.renderPlaylistList()}
        </Tab>
      </Tabs>
    );
  }
}

function mapStateToProps(state) {
  return {
    item: state.tracks.item,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleUpdateButtonClick: track => dispatch(creators.update.start(track)),
    handlePlaylistClick:     ({ id }) => dispatch(push({ pathname: `/playlists/${id}` })),
    handleArtistClick:       ({ id }) => dispatch(push({ pathname: `/artists/${id}` })),
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackDetail);
