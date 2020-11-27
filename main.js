import wasm from "./Cargo.toml";
import './index.css';

async function main() {
    const exports = await wasm();
    exports.main();
}
main()