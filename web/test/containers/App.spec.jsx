import React                    from 'react';
import assert                   from 'assert';
import { mount }                from 'enzyme';
import { MemoryRouter, Switch } from 'react-router-dom'
import MuiThemeProvider         from 'material-ui/styles/MuiThemeProvider';
import AppBar                   from 'material-ui/AppBar';
import Drawer                   from 'material-ui/Drawer';
import injectTapEventPlugin     from 'react-tap-event-plugin';
import createTestStore          from '../createTestStore';
import App                      from '../../js/containers/App';

describe('<App />', () => {
  it('render', () => {
    const wrapper = mount(
      <MuiThemeProvider>
        <MemoryRouter>
          <App store={createTestStore()}>
            <Switch />
          </App>
        </MemoryRouter>
      </MuiThemeProvider>
    );
    assert(wrapper.find(Drawer).length, 1);
    assert(wrapper.find(AppBar).length, 1);
  });
});
