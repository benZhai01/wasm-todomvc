const path = require('path');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require('copy-webpack-plugin');

module.exports = env => {
    
    let config = {
        entry: './src/js/index.js',
        output: {
            path: path.resolve(__dirname, 'dist'),
            filename: 'index.js',
        },
        plugins: [
            new HtmlWebpackPlugin({
                template: './index.html'
            }),
            new WasmPackPlugin({
                crateDirectory: path.resolve(__dirname, "."),
                args: '--log-level verbose'
            }),
            // Have this example work in Edge which doesn't ship `TextEncoder` or
            // `TextDecoder` at this time.
            new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
            })
        ],
        mode: 'development',
        devtool: 'source-map'
    };

    if(env == 'prod'){
        config.mode = 'production';
        config.devtool = 'none';
        config.output.path = path.resolve(__dirname, 'artifacts');
        config.plugins.push(new CopyPlugin({
            patterns: [
                {from: 'asset', to: 'asset'}
            ]
        }))
    }

    return config;
}
