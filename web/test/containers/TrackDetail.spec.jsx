import React              from 'react';
import assert             from 'assert';
import { shallow }        from 'enzyme';
import { Card }           from 'material-ui/Card';
import createTestStore    from '../createTestStore';
import TrackDetail        from '../../js/containers/TrackDetail';

describe('<TrackDetail />', () => {
  it('render', () => {
    const location = {query: { page: 0, perPage: 10 }};
    const params   = {track_id: 'track_id' };
    const store    = createTestStore();
    const wrapper  = shallow(
      <TrackDetail store={store} location={location} params={params}/>
    );
    assert(wrapper.dive().find(Card).length, 1);
  });
});
