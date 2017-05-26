import 'url-search-params-polyfill';
import React                from 'react';
import assert               from 'assert';
import { mount }            from 'enzyme';
import { MemoryRouter }     from 'react-router-dom'
import MuiThemeProvider     from 'material-ui/styles/MuiThemeProvider';
import { Table }            from 'material-ui/Table';
import createTestStore      from '../createTestStore';
import EntryList            from '../../js/containers/EntryList';

describe('<EntryList />', () => {
  it('render', () => {
    const params = new URLSearchParams();
    params.append('page', 0);
    params.append('per_page', 10);
    const location = { search: params.toString() };
    const store    = createTestStore();
    const wrapper  = mount(
      <MuiThemeProvider>
        <MemoryRouter>
          <EntryList store={store} location={location} />
        </MemoryRouter>
      </MuiThemeProvider>
    );
    assert(wrapper.find(EntryList).length, 1);
    assert(wrapper.find(Table).length, 1);
  });
});
