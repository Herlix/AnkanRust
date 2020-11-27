import {
    terser
} from 'rollup-plugin-terser';
import copy from 'rollup-plugin-copy'
import rust from "@wasm-tool/rollup-plugin-rust";
import serve from 'rollup-plugin-serve';
import del from 'rollup-plugin-delete'
import postcss from 'rollup-plugin-postcss'

const DIR = './__dist__/';
const isRelease = process.env.NODE_ENV === "release";

export default {
    watch: {
        include: ['src/**']
    },
    input: 'main.js',
    output: {
        file: DIR + 'bundle.js',
        format: 'iife',
        name: 'main',
        plugins: isRelease ? [terser()] : []
    },
    plugins: [
        postcss({
            sourceMap: isRelease,
            minimize: isRelease,
            extract: true,
        }),
        rust({
            debug: !isRelease,
            cargoArgs: isRelease ? ["--no-default-features", "--features", "wee_alloc"] : [],
            importHook: function (path) {
                return JSON.stringify('/' + path);
            },
        }),
        copy({
            targets: [{
                src: 'static/*',
                dest: DIR
            }]
        }),
        serve({
            open: false,
            contentBase: DIR,
            port: 8080,
            host: 'localhost',
            historyApiFallback: true,
            headers: {
                'Content-Type': 'application/wasm'
            },
        }),
        del({
            targets: DIR + 'assets/*.wasm'
        })
    ]
};