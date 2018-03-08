import React from 'react';
import PropTypes from 'prop-types';
import {
  Card,
  CardActions,
  CardHeader,
  CardMedia,
  CardTitle,
  CardText,
} from 'material-ui/Card';
import { PropertyList } from 'material-jsonschema';
import RaisedButton     from 'material-ui/RaisedButton';
import { connect }      from 'react-redux';
import { creators }     from '../actions/track';
import tryGet           from '../utils/tryGet';
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
    };
  }
  render() {
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
        <CardText>
          <PropertyList schema={schema} item={this.props.item} />
        </CardText>
      </Card>
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
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(TrackDetail);
