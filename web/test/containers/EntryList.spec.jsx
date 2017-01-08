import React              from 'react';
import assert             from 'assert';
import { shallow }        from 'enzyme';
import { Table }          from 'material-ui/Table';
import createTestStore    from '../createTestStore';
import EntryList          from '../../js/containers/EntryList';

describe('<EntryList />', () => {
  it('render', () => {
    const location = {query: { page: 0, perPage: 10}};
    const store    = createTestStore();
    const wrapper  = shallow(<EntryList store={store} location={location} />);
    assert(wrapper.dive().find(Table).length, 1);
  });
});
