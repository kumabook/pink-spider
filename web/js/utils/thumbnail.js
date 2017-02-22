export const NO_IMAGE   = '/web/no_image.png';
export const DEAD_IMAGE = '/web/dead_image.png';
const images = {
  YouTube:    '/web/youtube.png',
  SoundCloud: '/web/soundcloud.png',
  Spotify:    '/web/spotify.png',
  AppleMusic: '/web/apple_music.png',
};
export const getImageOfProvider = provider => images[provider] || NO_IMAGE;
