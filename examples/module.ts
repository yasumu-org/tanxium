// @ts-ignore
import { decode, encode } from "https://esm.run/js-base64@3.7.7";

const encoded = encode("Hello, world!");
console.log(encoded); // SGVsbG8sIHdvcmxkIQ==

const decoded = decode(encoded);
console.log(decoded); // Hello, world!
