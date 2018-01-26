const path = require("path");
const webpack = require("webpack");

module.exports = {
  devtool: 'source-map',
  entry: {
    vendor: [
      "axios", "material-ui", "moment",
      "react", "react-dom", "react-paginate", "react-redux",
      "react-router", "react-router-dom", "react-router-redux",
      "react-tap-event-plugin",
      "redux", "redux-saga", "redux-saga-router"
    ],
    app: "./web/js/index.jsx"
  },
  output: {
    path: path.resolve(__dirname, "public"),
    filename: "[name].bundle.js",
    publicPath: "./web/",
  },
  module: {
    rules: [
      {
        enforce: "pre",
        test: /\.jsx?$/,
        exclude: /node_modules/,
        loader: "eslint-loader",
        options: {
          configFile: '.eslintrc',
          fix: true,
        }
      },
      {
        test: /.jsx?$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'babel-loader'
      },
      {
        test: /\.css$/,
        loader: "style!css"
      },
    ]
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin({
      name: "vendor",
      minChunks: Infinity,
    })
  ],
  resolve: {
    extensions: ['.js', '.jsx'],
  }
};
