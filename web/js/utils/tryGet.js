export default (obj, key, def) => {
  if (obj && obj[key] !== undefined) {
    return obj[key];
  }
  return def;
};
