import React             from 'react';
import AppBar            from 'material-ui/AppBar';
import Drawer            from 'material-ui/Drawer';
import MenuItem          from 'material-ui/MenuItem';
import { connect }       from 'react-redux';
import { push }          from 'react-router-redux';
import { withRouter }    from 'react-router-dom';
import { toggleDrawler } from '../actions';

class App extends React.Component {
  static get propTypes() {
    return {
      children:                 React.PropTypes.element.isRequired,
      handleClick:              React.PropTypes.func.isRequired,
      handleEntriesMenuClick:   React.PropTypes.func.isRequired,
      handlePlaylistsMenuClick: React.PropTypes.func.isRequired,
      handleAlbumsMenuClick:    React.PropTypes.func.isRequired,
      handleTracksMenuClick:    React.PropTypes.func.isRequired,
      handleArtistsMenuClick:   React.PropTypes.func.isRequired,
      drawlerIsOpen:            React.PropTypes.bool.isRequired,
    };
  }
  render() {
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
      </div>
    );
  }
}

function mapStateToProps(state) {
  return {
    drawlerIsOpen: state.app.drawlerIsOpen,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleClick:            () => dispatch(toggleDrawler()),
    handleEntriesMenuClick: () => {
      dispatch(push({ pathname: 'entries', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handlePlaylistsMenuClick: () => {
      dispatch(push({ pathname: 'playlists', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleAlbumsMenuClick: () => {
      dispatch(push({ pathname: 'albums', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleTracksMenuClick: () => {
      dispatch(push({ pathname: 'tracks', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
    handleArtistsMenuClick: () => {
      dispatch(push({ pathname: 'artists', query: { page: 0 } }));
      dispatch(toggleDrawler());
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(App));
