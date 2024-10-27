test('Add two numbers', () => {
  expect(1 + 2).toBe(3);
});

test('Sub two numbers', () => {
  expect(1 - 2).toBe(4);
});

function danger() {
  throw new Error('Danger!');
}

test('Calls a function', () => {
  danger();
});

test('Calls a dangerous function', () => {
  expect(danger).toThrow();
});
