// @ts-nocheck
const message = "Hello, World!";

const encoded = btoa(message);
console.log("btoa", encoded);

const decoded = atob(encoded);
console.log("atob", decoded);
