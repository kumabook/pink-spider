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
import RaisedButton       from 'material-ui/RaisedButton';
import { PropertyList }   from 'material-jsonschema';
import { connect }        from 'react-redux';
import { update }         from '../actions/artist';
import { getUrl, schema } from '../model/Artist';
import tryGet             from '../utils/tryGet';

import {
  NO_IMAGE,
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
    const name        = tryGet(this.props.item, 'name', 'No Name');
    const description = tryGet(this.props.item, 'description', 'No Description');
    const provider    = tryGet(this.props.item, 'provider', 'No Service');
    const artworkUrl  = tryGet(this.props.item, 'artwork_url', NO_IMAGE);
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
          <img alt="artwork" src={artworkUrl} />
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
          {<PropertyList schema={schema} item={this.props.item} />}
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
