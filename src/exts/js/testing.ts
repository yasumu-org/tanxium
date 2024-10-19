interface AssertionData {
  expected: any;
  actual: any;
}

class AssertionError extends Error {
  data: AssertionData;

  constructor(message: string, data: AssertionData) {
    super(message);
    this.name = "AssertionError";
    this.data = data;
    Error.captureStackTrace(this, AssertionError);
  }

  getDiff(indent = 0) {
    const expected = `\x1b[31m- ${this.data.expected}\x1b[0m`;
    const actual = `\x1b[32m+ ${this.data.actual}\x1b[0m`;
    const idn = " ".repeat(indent);

    return `${expected}\n${idn}${actual}`;
  }
}

class Assertion {
  constructor(
    private value: any,
    private expectation?: any,
    private invert = false,
  ) {}

  get not() {
    return new Assertion(this.value, this.expectation, !this.invert);
  }

  toBe(expected: any) {
    if (this.invert ? this.value === expected : this.value !== expected) {
      throw new AssertionError(`Expected ${this.value} to be ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toEqual(expected: any) {
    if (this.invert ? this.value == expected : this.value != expected) {
      throw new AssertionError(`Expected ${this.value} to equal ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toStrictEqual(expected: any) {
    if (this.invert ? this.value === expected : this.value !== expected) {
      throw new AssertionError(
        `Expected ${this.value} to strictly equal ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toBeTruthy() {
    if (this.invert ? this.value : !this.value) {
      throw new AssertionError(`Expected ${this.value} to be truthy`, {
        expected: true,
        actual: this.value,
      });
    }
  }

  toBeFalsy() {
    if (this.invert ? !this.value : this.value) {
      throw new AssertionError(`Expected ${this.value} to be falsy`, {
        expected: false,
        actual: this.value,
      });
    }
  }

  toBeNull() {
    if (this.invert ? this.value === null : this.value !== null) {
      throw new AssertionError(`Expected ${this.value} to be null`, {
        expected: null,
        actual: this.value,
      });
    }
  }

  toBeUndefined() {
    if (this.invert ? this.value === undefined : this.value !== undefined) {
      throw new AssertionError(`Expected ${this.value} to be undefined`, {
        expected: undefined,
        actual: this.value,
      });
    }
  }

  toBeInstanceOf(expected: any) {
    if (
      this.invert
        ? this.value instanceof expected
        : !(this.value instanceof expected)
    ) {
      throw new AssertionError(
        `Expected ${this.value} to be an instance of ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toMatch(expected: any) {
    if (
      this.invert ? this.value.match(expected) : !this.value.match(expected)
    ) {
      throw new AssertionError(`Expected ${this.value} to match ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toThrow() {
    let error = null;

    try {
      this.value();
    } catch (e) {
      error = e;
    }

    if (this.invert ? error : !error) {
      throw new AssertionError(`Expected function to throw an error`, {
        expected: "Error",
        actual: error,
      });
    }
  }

  toThrowError(expected: any) {
    let error: Error | null = null;

    try {
      this.value();
    } catch (e: any) {
      error = e;
    }

    if (
      this.invert ? error instanceof expected : !(error instanceof expected)
    ) {
      throw new AssertionError(
        `Expected function to throw an instance of ${expected}`,
        {
          expected,
          actual: error,
        },
      );
    }
  }

  toHaveProperty(expected: any) {
    if (this.invert ? this.value[expected] : !this.value[expected]) {
      throw new AssertionError(`Expected object to have property ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toHaveLength(expected: any) {
    if (
      this.invert
        ? this.value.length === expected
        : this.value.length !== expected
    ) {
      throw new AssertionError(`Expected array to have length of ${expected}`, {
        expected,
        actual: this.value.length,
      });
    }
  }

  toContain(expected: any) {
    if (
      this.invert
        ? this.value.includes(expected)
        : !this.value.includes(expected)
    ) {
      throw new AssertionError(`Expected array to contain ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toContainEqual(expected: any) {
    if (
      this.invert
        ? this.value.includes(expected)
        : !this.value.includes(expected)
    ) {
      throw new AssertionError(`Expected array to contain ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toContainKey(expected: any) {
    if (this.invert ? expected in this.value : !(expected in this.value)) {
      throw new AssertionError(`Expected object to contain key ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toContainValue(expected: any) {
    if (
      this.invert
        ? Object.values(this.value).includes(expected)
        : !Object.values(this.value).includes(expected)
    ) {
      throw new AssertionError(`Expected object to contain value ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toContainEntry(expected: any) {
    const [key, value] = expected;

    if (this.invert ? this.value[key] === value : this.value[key] !== value) {
      throw new AssertionError(`Expected object to contain entry ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toContainEqualEntry(expected: any) {
    const [key, value] = expected;

    if (this.invert ? this.value[key] == value : this.value[key] != value) {
      throw new AssertionError(`Expected object to contain entry ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }

  toBeGreaterThan(expected: any) {
    if (this.invert ? this.value > expected : this.value <= expected) {
      throw new AssertionError(
        `Expected ${this.value} to be greater than ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toBeGreaterThanOrEqual(expected: any) {
    if (this.invert ? this.value >= expected : this.value < expected) {
      throw new AssertionError(
        `Expected ${this.value} to be greater than or equal to ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toBeLessThan(expected: any) {
    if (this.invert ? this.value < expected : this.value >= expected) {
      throw new AssertionError(
        `Expected ${this.value} to be less than ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toBeLessThanOrEqual(expected: any) {
    if (this.invert ? this.value <= expected : this.value > expected) {
      throw new AssertionError(
        `Expected ${this.value} to be less than or equal to ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toBeCloseTo(expected: any, delta: number) {
    if (
      this.invert
        ? Math.abs(this.value - expected) <= delta
        : Math.abs(this.value - expected) > delta
    ) {
      throw new AssertionError(
        `Expected ${this.value} to be close to ${expected}`,
        {
          expected,
          actual: this.value,
        },
      );
    }
  }

  toHaveLengthOf(expected: any) {
    if (
      this.invert
        ? this.value.length === expected
        : this.value.length !== expected
    ) {
      throw new AssertionError(`Expected array to have length of ${expected}`, {
        expected,
        actual: this.value.length,
      });
    }
  }

  toHavePropertyOf(expected: any) {
    if (this.invert ? this.value[expected] : !this.value[expected]) {
      throw new AssertionError(`Expected object to have property ${expected}`, {
        expected,
        actual: this.value,
      });
    }
  }
}

function test(description: string, fn: () => void) {
  try {
    fn();
    console.log(`✅ ${description}`);
  } catch (error: any) {
    if (error instanceof AssertionError) {
      console.log(
        `❌ ${description}\n   ${error.message}\n   ${error.getDiff(3)}`,
      );
    } else {
      console.log(`❌ ${description}\n  ${error.stack}`);
    }
  }
}

function expect(value: any) {
  return new Assertion(value);
}

Object.assign(globalThis, {
  test,
  it: test,
  expect,
  Assertion,
  AssertionError,
});
