// @ts-nocheck

const message = 'Hello, World!';

const encoded = btoa(message);
console.log('btoa', encoded);

const decoded = atob(encoded);
console.log('atob', decoded);

const encoded2 = Base64.encode(message);

console.log('Base64.encode', encoded2);
console.log('Base64.decode', Base64.decode(encoded2));

const encoded3 = Base64.encodeURL(message);

console.log('Base64.encodeURL', encoded3);
console.log('Base64.decodeURL', Base64.decodeURL(encoded3));
