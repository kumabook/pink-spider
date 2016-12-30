const path = require("path");
module.exports = {
  devtool: 'source-map',
  entry: "./web/js/index.jsx",
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "bundle.js",
    publicPath: "bundle.js",
  },
  module: {
    preLoaders: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        loader: "eslint-loader"
      }
    ],
    loaders: [
      {
        test: /.jsx?$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'babel-loader',
        query: {
          presets: ['es2015', 'react', 'stage-1']
        }
      },
      { test: /\.css$/, loader: "style!css" }
    ]
  },
  eslint: {
    configFile: '.eslintrc',
    fix: true,
  },
  resolve: {
    extensions: ['', '.js', '.jsx'],
  }
};
