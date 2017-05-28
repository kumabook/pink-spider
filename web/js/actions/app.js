export const toggleDrawler = () => ({ type: 'TOGGLE_DRAWLER' });
export const showProgress  = () => ({ type: 'SHOW_PROGRESS' });
export const hideProgress  = () => ({ type: 'HIDE_PROGRESS' });
export const showMessage   = payload => ({ type: 'SHOW_MESSAGE', payload });
export const closeMessage  = () => ({ type: 'CLOSE_MESSAGE' });
