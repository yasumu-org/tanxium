import {
    op_generate_nanoid,
    op_generate_ulid,
    op_tanxium_version
} from 'ext:core/ops';

// Define Tanxium global object
const Tanxium = Object.assign({}, Deno, {
    version: {
        ...Deno.version,
        tanxium: op_tanxium_version(),
    },
    uuid() {
        return crypto.randomUUID();
    },
    nanoid() {
        return op_generate_nanoid();
    },
    ulid() {
        return op_generate_ulid();
    }
})

if (typeof 'process' !== 'undefined' && 'versions' in process) {
    process.versions.tanxium = Tanxium.version.tanxium;
}

// Define Tanxium global object
Object.defineProperty(globalThis, "Tanxium", {
    value: Tanxium,
    writable: false,
    configurable: false,
    enumerable: true,
});

Object.assign(globalThis, {
    test: function () { },
    expect: function () { },
    AssertionError: Error,
});