import { greet } from './pkg/scrypt_rs_wasm.js';

const res = greet('World');
console.dir(res);