import React             from 'react';
import PropTypes         from 'prop-types';
import AppBar            from 'material-ui/AppBar';
import Drawer            from 'material-ui/Drawer';
import MenuItem          from 'material-ui/MenuItem';
import CircularProgress  from 'material-ui/CircularProgress';
import { connect }       from 'react-redux';
import { push }          from 'react-router-redux';
import { withRouter }    from 'react-router-dom';
import { toggleDrawler } from '../actions';

const progressStyle = {
  position:        'absolute',
  top:             0,
  left:            0,
  width:           '100%',
  height:          '100%',
  backgroundColor: 'rgba(10, 10, 10, 0.4)',
  zIndex:          2000,
};

class App extends React.Component {
  static get propTypes() {
    return {
      children:                 PropTypes.element.isRequired,
      handleClick:              PropTypes.func.isRequired,
      handleEntriesMenuClick:   PropTypes.func.isRequired,
      handlePlaylistsMenuClick: PropTypes.func.isRequired,
      handleAlbumsMenuClick:    PropTypes.func.isRequired,
      handleTracksMenuClick:    PropTypes.func.isRequired,
      handleArtistsMenuClick:   PropTypes.func.isRequired,
      drawlerIsOpen:            PropTypes.bool.isRequired,
      progress:                 PropTypes.bool.isRequired,
    };
  }
  render() {
    const progress = !this.props.progress ? null : (
      <div style={progressStyle}>
        <CircularProgress style={{ marginTop: '45%', marginLeft: '49%' }} />
      </div>
    );
    return (
      <div>
        <AppBar
          title="pink spider"
          iconClassNameRight="muidocs-icon-navigation-expand-more"
          onLeftIconButtonTouchTap={this.props.handleClick}
        />
        <Drawer open={this.props.drawlerIsOpen}>
          <AppBar
            title="Menu"
            onLeftIconButtonTouchTap={this.props.handleClick}
          />
          <MenuItem onTouchTap={this.props.handleEntriesMenuClick}>Entries</MenuItem>
          <MenuItem onTouchTap={this.props.handlePlaylistsMenuClick}>Playlists</MenuItem>
          <MenuItem onTouchTap={this.props.handleAlbumsMenuClick}>Albums</MenuItem>
          <MenuItem onTouchTap={this.props.handleTracksMenuClick}>Tracks</MenuItem>
          <MenuItem onTouchTap={this.props.handleArtistsMenuClick}>Artists</MenuItem>
        </Drawer>
        {this.props.children}
        {progress}
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    title:         state.app.title,
    drawlerIsOpen: state.app.drawlerIsOpen,
    message:       state.app.message,
    progress:      state.app.progress,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleClick:            () => dispatch(toggleDrawler()),
    handleEntriesMenuClick: () => {
      dispatch(push({ pathname: '/entries', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handlePlaylistsMenuClick: () => {
      dispatch(push({ pathname: '/playlists', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleAlbumsMenuClick: () => {
      dispatch(push({ pathname: '/albums', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleTracksMenuClick: () => {
      dispatch(push({ pathname: '/tracks', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleArtistsMenuClick: () => {
      dispatch(push({ pathname: '/artists', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(App));
