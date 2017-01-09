export default (obj, key, def) => {
  if (obj && obj[key] !== undefined && obj[key] !== null) {
    return obj[key];
  }
  return def;
};
