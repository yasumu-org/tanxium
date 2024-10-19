import { strictEqual } from 'node:assert/strict';

Deno.test('Add two numbers', () => {
  strictEqual(1 + 2, 3);
});
