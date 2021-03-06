import React from 'react';
import PropTypes from 'prop-types';
import { withRouter } from 'react-router-dom';
import { push }       from 'react-router-redux';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import { Tabs, Tab }      from 'material-ui/Tabs';
import RaisedButton       from 'material-ui/RaisedButton';
import { List, ListItem } from 'material-ui/List';
import ContentLink        from 'material-ui/svg-icons/content/link';
import { PropertyList }   from 'material-jsonschema';
import { connect }        from 'react-redux';
import { creators }       from '../actions/playlist';
import tryGet             from '../utils/tryGet';
import { getUrl, schema } from '../model/Playlist';
import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class PlaylistDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    PropTypes.object.isRequired,
      handleUpdateButtonClick: PropTypes.func.isRequired,
      handleTrackClick:        PropTypes.func.isRequired,
    };
  }
  renderSummaryCard() {
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const title       = tryGet(this.props.item, 'title', 'No Title');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
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
          <img alt="artwork" src={state === 'alive' ? artworkUrl : DEAD_IMAGE} />
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
    const items = this.props.item.tracks.map(({ updated_at: updatedAt, track }, index) => (
      <ListItem
        key={track.id}
        primaryText={`${index + 1} ${track.title}`}
        secondaryText={updatedAt}
        rightIcon={<ContentLink onClick={() => this.props.handleTrackClick(track)} />}
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
        <Tab label="Tracks" >
          {this.renderTrackList()}
        </Tab>
      </Tabs>
    );
  }
}

function mapStateToProps(state) {
  return {
    item: state.playlists.item,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleUpdateButtonClick: item => dispatch(creators.update.start(item)),
    handleTrackClick:        ({ id }) => dispatch(push({ pathname: `/tracks/${id}` })),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(PlaylistDetail));
