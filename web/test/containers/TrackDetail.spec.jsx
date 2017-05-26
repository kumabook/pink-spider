import 'url-search-params-polyfill';
import React            from 'react';
import assert           from 'assert';
import { mount }        from 'enzyme';
import { MemoryRouter } from 'react-router-dom'
import MuiThemeProvider from 'material-ui/styles/MuiThemeProvider';
import { Card }         from 'material-ui/Card';
import createTestStore  from '../createTestStore';
import TrackDetail      from '../../js/containers/TrackDetail';
import track            from '../../js/api/track';
import nisemono         from 'nisemono';

describe('<TrackDetail />', () => {
  let show = null;
  it('render', () => {
    const params = new URLSearchParams();
    params.append('page', 0);
    params.append('per_page', 10);
    const location = { search: params.toString() };
    const store    = createTestStore();
    const match = {
      params: { track_id: 'abcdefg' }
    };
    const wrapper  = mount(
      <MuiThemeProvider>
        <MemoryRouter>
          <TrackDetail store={store} location={location} match={match} />
        </MemoryRouter>
      </MuiThemeProvider>
    );
    assert(wrapper.find(TrackDetail).length, 1);
    assert(wrapper.find(Card).length, 1);
  });
});
