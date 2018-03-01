import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
} from 'material-ui/Card';
import { Tabs, Tab }                 from 'material-ui/Tabs';
import { List, ListItem }          from 'material-ui/List';
import RaisedButton                from 'material-ui/RaisedButton';
import { connect }                 from 'react-redux';
import { creators }                from '../actions/album';
import { getUrl, getOwnerUrl }     from '../model/Album';
import tryGet                      from '../utils/tryGet';
import datePrettify                from '../utils/datePrettify';

import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class AlbumDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    PropTypes.object.isRequired,
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
  renderTrackList() {
    if (!this.props.item.tracks) {
      return null;
    }
    const items = this.props.item.tracks.map((track, index) => (
      <ListItem primaryText={`${index + 1} ${track.title}`} secondaryText={track.id} />
    ));
    return <List>{items}</List>;
  }
  renderPropsList() {
    const id          = tryGet(this.props.item, 'id', 'unknown id');
    const title       = tryGet(this.props.item, 'title', 'No Title');
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const identifier  = tryGet(this.props.item, 'identifier', 'No ID');
    const ownerId     = tryGet(this.props.item, 'owner_id', 'unknown');
    const ownerName   = tryGet(this.props.item, 'owner_name', 'unknown');
    const publishedAt = datePrettify(tryGet(this.props.item, 'published_at', null));
    const createdAt   = datePrettify(tryGet(this.props.item, 'created_at', null));
    const updatedAt   = datePrettify(tryGet(this.props.item, 'updated_at', null));

    return (
      <List>
        <ListItem primaryText="id" secondaryText={id} />
        <ListItem primaryText="title" secondaryText={title} />
        <ListItem primaryText="state" secondaryText={state} />
        <ListItem primaryText="provider" secondaryText={provider} />
        <ListItem primaryText="identifier" secondaryText={identifier} />
        <ListItem primaryText="owner id" secondaryText={ownerId} />
        <ListItem primaryText="owner name" secondaryText={ownerName} />
        <ListItem primaryText="published" secondaryText={publishedAt} />
        <ListItem primaryText="created" secondaryText={createdAt} />
        <ListItem primaryText="updated" secondaryText={updatedAt} />
      </List>
    );
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
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(AlbumDetail));
