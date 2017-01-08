import React              from 'react';
import assert             from 'assert';
import { shallow }        from 'enzyme';
import { Table }          from 'material-ui/Table';
import createTestStore    from '../createTestStore';
import TrackList          from '../../js/containers/TrackList';

describe('<TrackList />', () => {
  it('render', () => {
    const location = {query: { page: 0, perPage: 10 }};
    const params   = {entry_id: 'entry_id' };
    const store    = createTestStore();
    const wrapper  = shallow(
      <TrackList store={store} location={location} params={params}/>
    );
    assert(wrapper.dive().find(Table).length, 1);
  });
});
