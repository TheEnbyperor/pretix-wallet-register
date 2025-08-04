const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
    entry: "./bootstrap.js",
    module: {
        rules: [{
            test: /\.(js|jsx)$/,
            exclude: /node_modules/,
            use: ['babel-loader'],
        }, {
            test: /\.css$/i,
            use: ["style-loader", "css-loader"],
        }]
    },
    resolve: {
        extensions: ['*', '.js', '.jsx'],
    },
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CopyWebpackPlugin(["index.html", require.resolve("barkoder-wasm/barkoder.wasm")]),
    ],
    experiments: {
        asyncWebAssembly: true
    },
    devServer: {
        allowedHosts: "all"
    }
};
