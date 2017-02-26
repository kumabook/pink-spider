import React             from 'react';
import AppBar            from 'material-ui/AppBar';
import Drawer            from 'material-ui/Drawer';
import MenuItem          from 'material-ui/MenuItem';
import { connect }       from 'react-redux';
import { push }          from 'react-router-redux';
import { toggleDrawler } from '../actions';

class App extends React.Component {
  static get propTypes() {
    return {
      children:                 React.PropTypes.element,
      handleClick:              React.PropTypes.func,
      handleEntriesMenuClick:   React.PropTypes.func,
      handlePlaylistsMenuClick: React.PropTypes.func,
      handleAlbumsMenuClick:    React.PropTypes.func,
      handleTracksMenuClick:    React.PropTypes.func,
      drawlerIsOpen:            React.PropTypes.bool,
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
  };
}

export default connect(mapStateToProps, mapDispatchToProps)(App);
