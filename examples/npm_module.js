import { v4 } from "npm:uuid";
import { decode, encode } from "https://esm.run/js-base64@3.7.7";

console.log(encode("Hello, world!"));
console.log(decode("SGVsbG8sIHdvcmxkIQ=="));
console.log(v4());
