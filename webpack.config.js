const path = require('path')
const html = require('html-webpack-plugin')
const copy = require('copy-webpack-plugin')

module.exports = {
  entry: './static/index.js',
  output: {
    path: path.resolve(__dirname, './static/dist'),
    filename: 'bundle.js'
  },
  plugins: [
    new html({
      template: './static/index.html',
      minify: true
    }),
    new copy({
      patterns: [
        { from: './static/assets', to: './assets' }
      ]
    })
  ],
  devServer: {
    static: path.resolve(__dirname, './static/dist'),
    compress: true,
    port: 3000
  },
  devtool: 'source-map',
  module: {
    rules: [
      {
        test: /\.js$/,
        use: 'babel-loader',
        exclude: /node_modules/
      },
      {
        test: /\.svelte$/,
        use: 'svelte-loader'
      },
      {
        test: /\.css$/,
        use: ['style-loader', 'css-loader', 'postcss-loader']
      },
      {
        test: /\.(svg)/,
        use: 'raw-loader'
      }
    ]
  }
}