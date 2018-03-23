import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import { push } from 'react-router-redux';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
} from 'material-ui/Card';
import { Tabs, Tab }           from 'material-ui/Tabs';
import { List, ListItem }      from 'material-ui/List';
import ContentLink             from 'material-ui/svg-icons/content/link';
import RaisedButton            from 'material-ui/RaisedButton';
import { PropertyList }        from 'material-jsonschema';
import { connect }             from 'react-redux';
import { creators }            from '../actions/album';
import tryGet                  from '../utils/tryGet';
import {
  getUrl,
  getOwnerUrl,
  schema,
} from '../model/Album';

import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class AlbumDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    PropTypes.object.isRequired,
      handleArtistClick:       PropTypes.func.isRequired,
      handleUpdateButtonClick: PropTypes.func.isRequired,
    };
  }
  renderOwnerLink() {
    const ownerUrl = getOwnerUrl(this.props.item);
    return <a href={ownerUrl}>{tryGet(this.props.item, 'owner_name', 'Unknown')}</a>;
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
    return (
      <Card>
        <CardHeader
          title={this.renderOwnerLink()}
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
      </Card>
    );
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
  renderTrackList() {
    if (!this.props.item.tracks) {
      return null;
    }
    const items = this.props.item.tracks.map((track, index) => (
      <ListItem
        key={track.id}
        primaryText={`${index + 1} ${track.title}`}
        secondaryText={track.id}
      />
    ));
    return <List>{items}</List>;
  }
  renderPropsList() {
    return <PropertyList schema={schema} item={this.props.item} />;
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
        <Tab label="Tracks" >
          {this.renderTrackList()}
        </Tab>
      </Tabs>
    );
  }
}

function mapStateToProps(state) {
  return {
    item: state.albums.item,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleUpdateButtonClick: item => dispatch(creators.update.start(item)),
    handleArtistClick:       ({ id }) => dispatch(push({ pathname: `/artists/${id}` })),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(AlbumDetail));
