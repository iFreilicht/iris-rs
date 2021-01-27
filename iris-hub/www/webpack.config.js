const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: "./src/bootstrap.js",
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bootstrap.js",
  },
  experiments: {
    syncWebAssembly: true, // Deprecated, async needs research
  },
  mode: "development",
  plugins: [
    new CopyWebpackPlugin(['index.html', 'assets'])
  ],
};
