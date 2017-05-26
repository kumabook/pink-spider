import React            from 'react';
import assert           from 'assert';
import { mount }        from 'enzyme';
import { MemoryRouter } from 'react-router-dom'
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import { Table }        from 'material-ui/Table';
import createTestStore  from '../createTestStore';
import TrackList        from '../../js/containers/TrackList';

describe('<TrackList />', () => {
  it('render', () => {
    const location = {query: { page: 0, perPage: 10 }};
    const store    = createTestStore();
    const match = {
      params: { entry_id: 'entry_id' }
    };
    const wrapper  = mount(
      <MuiThemeProvider>
        <MemoryRouter>
          <TrackList store={store} location={location} match={match}/>
        </MemoryRouter>
      </MuiThemeProvider>
    );
    assert.strictEqual(wrapper.find(TrackList).length, 1);
    assert.strictEqual(wrapper.find(Table).length, 1);
  });
});
