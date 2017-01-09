export const getOwnerUrl = (track) => {
  if (!track || !track.owner_id) {
    return null;
  }
  const ownerId = track.owner_id;
  switch (track.provider) {
    case 'YouTube':
      return `https://www.youtube.com/channel/${ownerId}`;
    case 'SoundCloud':
      return null; // Not support
    default:
      return null;
  }
};

export const getUrl = (track) => {
  if (track.url) {
    return track.url;
  }
  if (!track || !track.identifier) {
    return null;
  }
  const id = track.identifier;
  switch (track.provider) {
    case 'YouTube':
      return `https://www.youtube.com/watch/?v=${id}`;
    case 'SoundCloud':
      return `https://soundcloud.com/tracks/${id}`;
    default:
      return null;
  }
};
