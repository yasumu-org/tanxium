import { setTimeout } from 'node:timers/promises';

console.log(process.versions);
console.log('Fetching data...');

await setTimeout(2000);

console.log('Data fetched!');