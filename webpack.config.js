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
  resolve: {
    extensions: ['.js', '.jsx'],
  }
};
