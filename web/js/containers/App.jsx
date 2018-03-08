import React             from 'react';
import PropTypes         from 'prop-types';
import AppBar            from 'material-ui/AppBar';
import { getStyles }     from 'material-ui/AppBar/AppBar';
import Drawer            from 'material-ui/Drawer';
import MenuItem          from 'material-ui/MenuItem';
import CircularProgress  from 'material-ui/CircularProgress';
import FlatButton        from 'material-ui/FlatButton';
import Search            from 'material-ui/svg-icons/action/search';
import TextField         from 'material-ui/TextField';
import { connect }       from 'react-redux';
import { push }          from 'react-router-redux';
import { withRouter }    from 'react-router-dom';
import { toggleDrawler } from '../actions';

const progressStyle = {
  position:        'fixed',
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
      handleFeedsMenuClick:     PropTypes.func.isRequired,
      handleEntriesMenuClick:   PropTypes.func.isRequired,
      handlePlaylistsMenuClick: PropTypes.func.isRequired,
      handleAlbumsMenuClick:    PropTypes.func.isRequired,
      handleTracksMenuClick:    PropTypes.func.isRequired,
      handleArtistsMenuClick:   PropTypes.func.isRequired,
      handleSearch:             PropTypes.func.isRequired,
      drawlerIsOpen:            PropTypes.bool.isRequired,
      progress:                 PropTypes.bool.isRequired,
      needSearch:               PropTypes.bool.isRequired,
      location:                 PropTypes.object.isRequired,
    };
  }
  static get contextTypes() {
    return { muiTheme: React.PropTypes.object.isRequired };
  }
  constructor() {
    super();
    this.handleSearchFormSubmit = this.handleSearchFormSubmit.bind(this);
  }
  handleSearchFormSubmit(e) {
    e.preventDefault();
    const query        = e.target.query.value;
    const { pathname } = this.props.location;
    if (e.target.query.value) {
      this.props.handleSearch(query, pathname);
    }
  }
  renderIconElementRight() {
    if (!this.props.needSearch) {
      return null;
    }
    const styles                = getStyles(this.props, this.context);
    styles.flatButton.top       = styles.flatButton.marginTop;
    styles.flatButton.marginTop = 0;
    const textFieldStyle = {
      color: '#FFFFFF',
    };
    const hintStyle = {
      color: '#DDDDDD',
    };
    return (
      <form onSubmit={this.handleSearchFormSubmit}>
        <TextField
          name="query"
          hintText="Search"
          inputStyle={textFieldStyle}
          hintStyle={hintStyle}
        />
        <FlatButton
          type="submit"
          style={styles.flatButton}
        >
          <Search color="#FFFFFF" />
        </FlatButton>
      </form>
    );
  }
  render() {
    const progress = !this.props.progress ? null : (
      <div style={progressStyle}>
        <CircularProgress style={{ marginTop: '30%', marginLeft: '49%' }} />
      </div>
    );
    return (
      <div>
        <AppBar
          title="pink spider"
          iconElementRight={this.renderIconElementRight()}
          onLeftIconButtonTouchTap={this.props.handleClick}
        />
        <Drawer open={this.props.drawlerIsOpen}>
          <AppBar
            title="Menu"
            onLeftIconButtonTouchTap={this.props.handleClick}
          />
          <MenuItem onTouchTap={this.props.handleFeedsMenuClick}>Feeds</MenuItem>
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
    needSearch:    state.app.needSearch,
  };
}

function mapDispatchToProps(dispatch) {
  return {
    handleClick:          () => dispatch(toggleDrawler()),
    handleFeedsMenuClick: () => {
      dispatch(push({ pathname: '/feeds' }));
      dispatch(toggleDrawler());
    },
    handleEntriesMenuClick: () => {
      dispatch(push({ pathname: '/entries' }));
      dispatch(toggleDrawler());
    },
    handlePlaylistsMenuClick: () => {
      dispatch(push({ pathname: '/playlists' }));
      dispatch(toggleDrawler());
    },
    handleAlbumsMenuClick: () => {
      dispatch(push({ pathname: '/albums' }));
      dispatch(toggleDrawler());
    },
    handleTracksMenuClick: () => {
      dispatch(push({ pathname: '/tracks' }));
      dispatch(toggleDrawler());
    },
    handleArtistsMenuClick: () => {
      dispatch(push({ pathname: '/artists' }));
      dispatch(toggleDrawler());
    },
    handleSearch: (query, pathname = '/tracks') => {
      dispatch(push({ pathname, search: `query=${query}` }));
    },
  };
}

export default withRouter(connect(mapStateToProps, mapDispatchToProps)(App));
