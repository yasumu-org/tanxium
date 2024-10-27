/// <reference path="./types.d.ts" />

import {
  op_generate_nanoid,
  op_generate_ulid,
  op_tanxium_version,
} from 'ext:core/ops';
import process from 'node:process';

let TanxiumRuntimeData: Record<string, unknown> = {};

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
  },
  getRuntimeDataString() {
    return JSON.stringify(Tanxium.getRuntimeData());
  },
  getRuntimeData() {
    const data = TanxiumRuntimeData;

    if (data === undefined) {
      const data = (TanxiumRuntimeData = {});
      return data;
    }

    return data;
  },
  setRuntimeData(value: Record<string, unknown>) {
    if (!value || typeof value !== 'object') {
      throw new TypeError('Invalid runtime data, expected an object');
    }

    TanxiumRuntimeData = value;
  },
  clearRuntimeData() {
    TanxiumRuntimeData = {};
  },
});

if (typeof 'process' !== 'undefined' && 'versions' in process) {
  process.versions.tanxium = Tanxium.version.tanxium;
}

// Define Tanxium global object
Object.defineProperty(globalThis, 'Tanxium', {
  value: Tanxium,
  writable: false,
  configurable: false,
  enumerable: true,
});

Object.assign(globalThis, {
  test: function () {},
  expect: function () {},
  AssertionError: Error,
});
