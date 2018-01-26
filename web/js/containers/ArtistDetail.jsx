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
import { List, ListItem }          from 'material-ui/List';
import RaisedButton                from 'material-ui/RaisedButton';
import { connect }                 from 'react-redux';
import { update }                  from '../actions/artist';
import { getUrl }                  from '../model/Artist';
import tryGet                      from '../utils/tryGet';
import datePrettify                from '../utils/datePrettify';

import {
  NO_IMAGE,
  DEAD_IMAGE,
  getImageOfProvider,
} from '../utils/thumbnail';

class ArtistDetail extends React.Component {
  static get propTypes() {
    return {
      item:                    PropTypes.object.isRequired,
      handleUpdateButtonClick: PropTypes.func.isRequired,
    };
  }
  render() {
    const id          = tryGet(this.props.item, 'id', 'unknown id');
    const state       = tryGet(this.props.item, 'state', 'unknown state');
    const name        = tryGet(this.props.item, 'name', 'No Name');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const identifier  = tryGet(this.props.item, 'identifier', 'No ID');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
    const publishedAt = datePrettify(tryGet(this.props.item, 'published_at', null));
    const createdAt   = datePrettify(tryGet(this.props.item, 'created_at', null));
    const updatedAt   = datePrettify(tryGet(this.props.item, 'updated_at', null));
    const overlay = (
      <CardTitle
        title={name}
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
          title={name}
          subtitle={this.props.item.url}
          avatar={getImageOfProvider(provider)}
        />
        <CardMedia style={style} overlay={overlay} >
          <img alt="artwork" src={state === 'alive' ? artworkUrl : DEAD_IMAGE} />
        </CardMedia>
        <CardTitle title={name} />
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
        <CardText>
          <List>
            <ListItem primaryText="id" secondaryText={id} />
            <ListItem primaryText="name" secondaryText={name} />
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
    item: state.artists.item,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleUpdateButtonClick: item => dispatch(update(item)),
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(ArtistDetail));
