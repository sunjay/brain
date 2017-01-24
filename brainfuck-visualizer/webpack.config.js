const path = require('path');

const webpack = require('webpack');
const autoprefixer = require('autoprefixer');

module.exports = {
  entry: ['./app/index.jsx'],
  target: 'electron-renderer',
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
    publicPath: 'http://localhost:8080/',
  },
  module: {
    loaders: [
      {
        test: /\.jsx?$/,
        exclude: /(node_modules|bower_components)/,
        loaders: ['babel-loader'],
      },
      {
        test: /\.scss$/,
        loaders: ['style', 'css?modules&importLoaders=1&sourceMap!postcss!sass?sourceMap&sourceMapContents'],
      },
      {
        test: /\.json$/,
        exclude: /(node_modules|bower_components)/,
        loader: 'json-loader',
      },
      {
        test: /\.(eot|woff|woff2|ttf|svg|png|jpe?g|gif)(\?\S*)?$/,
        loader: 'url?limit=100000&name=[name].[ext]',
      },
    ],
  },
  plugins: [
    new webpack.DefinePlugin({
      'process.env': {
        'NODE_ENV': JSON.stringify(process.env.NODE_ENV),
        'APP_ROOT': JSON.stringify(__dirname),
        'APP_VERSION': JSON.stringify(require('./package.json').version),
      },
    }),
  ],
  resolve: {
    extensions: ['', '.js', '.jsx'],
    alias: {
      'image-assets': path.join(__dirname, 'assets/images'),
    },
  },
  devtool: 'cheap-module-eval-source-map',
  postcss: [autoprefixer],
};
