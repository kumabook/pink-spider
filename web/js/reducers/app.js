const initialState = { drawlerIsOpen: false };

export default function toggleDrawler(state = initialState, action) {
  if (action.type === 'TOGGLE_DRAWLER') {
    return { drawlerIsOpen: !state.drawlerIsOpen };
  }
  return state;
}
