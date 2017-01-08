import React              from 'react';
import assert             from 'assert';
import { shallow }        from 'enzyme';
import AppBar             from 'material-ui/AppBar';
import Drawer             from 'material-ui/Drawer';
import createTestStore    from '../createTestStore';
import App                from '../../js/containers/App';

describe('<App />', () => {
  it('render', () => {
    const wrapper = shallow(<App store={createTestStore()} />);
    assert(wrapper.dive().find(Drawer).length, 1);
    assert(wrapper.dive().find(AppBar).length, 1);
  });
});
