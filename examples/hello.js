function add(a, b) {
    return a + b;
}

test("should add two numbers", () => {
    expect(add(1, 2)).toBe(3);
});

test("should add two numbers again", () => {
    expect(add(1, 2)).not.toBe(3);
});