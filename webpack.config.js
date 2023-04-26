const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const webpack = require('webpack');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js'
        // clean: true
    },
    plugins: [
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, ".")
        }),
        // Have this example work in Edge which doesn't ship `TextEncoder` or
        // `TextDecoder` at this time.
        new webpack.ProvidePlugin({
            TextDecoder: ['text-encoding', 'TextDecoder'],
            TextEncoder: ['text-encoding', 'TextEncoder']
        }),
        new CopyWebpackPlugin({
            patterns: [
                {from: 'web'}]
        })
    ],
    module: {
        rules: [
            {
                test: /index_bg\.js$/,
                loader: "string-replace-loader",
                options: {
                    multiple: [
                        {search: "import * as __wbg_star0 from 'env';", replace: ""},
                        {search: "let wasm;", replace: "let wasm; export const set_wasm = (w) => wasm = w;"},
                        {search: "imports['env'] = __wbg_star0;", replace: "return imports.wbg;"},
                        {search: "const imports = getImports();", replace: "return getImports();"},
                    ]
                }
            }
        ]
    },
    mode: "production",
    experiments: {
        asyncWebAssembly: true
    }
};
