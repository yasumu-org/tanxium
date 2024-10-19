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
    const { expected, actual } = this.data;
    const idn = " ".repeat(indent);
    const formattedExpected = `\x1b[31m- Expected: ${JSON.stringify(
      expected,
      null,
      2
    )}\x1b[0m`;
    const formattedActual = `\x1b[32m+ Actual: ${JSON.stringify(
      actual,
      null,
      2
    )}\x1b[0m`;

    return `${formattedExpected}\n${idn}${formattedActual}`;
  }
}

class Assertion {
  constructor(
    private value: any,
    private expectation?: any,
    private invert = false
  ) {}

  get not() {
    return new Assertion(this.value, this.expectation, !this.invert);
  }

  evaluate(condition, errorMessage, data) {
    if (this.invert ? condition : !condition) {
      throw new AssertionError(errorMessage, data);
    }
  }

  toBe(expected: any) {
    this.evaluate(
      this.value === expected,
      `Expected ${this.value} to be ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toEqual(expected: any) {
    this.evaluate(
      this.value == expected,
      `Expected ${this.value} to equal ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toStrictEqual(expected: any) {
    this.evaluate(
      this.value === expected,
      `Expected ${this.value} to strictly equal ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeTruthy() {
    this.evaluate(this.value, `Expected ${this.value} to be truthy`, {
      expected: true,
      actual: this.value,
    });
  }

  toBeFalsy() {
    this.evaluate(!this.value, `Expected ${this.value} to be falsy`, {
      expected: false,
      actual: this.value,
    });
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
    this.evaluate(
      this.value === undefined,
      `Expected ${this.value} to be undefined`,
      {
        expected: undefined,
        actual: this.value,
      }
    );
  }

  toBeInstanceOf(expected: any) {
    this.evaluate(
      this.value instanceof expected,
      `Expected ${this.value} to be an instance of ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toMatch(expected: any) {
    this.evaluate(
      this.value.match(expected),
      `Expected ${this.value} to match ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toThrow() {
    let error = null;

    try {
      this.value();
    } catch (e) {
      error = e;
    }

    this.evaluate(error, `Expected function to throw an error`, {
      expected: "Error",
      actual: error,
    });
  }

  toThrowError(expected: any) {
    let error: Error | null = null;

    try {
      this.value();
    } catch (e: any) {
      error = e;
    }

    this.evaluate(
      error instanceof expected,
      `Expected function to throw an instance of ${expected}`,
      {
        expected,
        actual: error,
      }
    );
  }

  toHaveProperty(expected: any) {
    this.evaluate(
      this.value[expected],
      `Expected object to have property ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toHaveLength(expected: any) {
    this.evaluate(
      this.value.length === expected,
      `Expected array to have length of ${expected}`,
      {
        expected,
        actual: this.value.length,
      }
    );
  }

  toContain(expected: any) {
    this.evaluate(
      this.value.includes(expected),
      `Expected array to contain ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toContainEqual(expected: any) {
    this.evaluate(
      this.value.includes(expected),
      `Expected array to contain ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toContainKey(expected: any) {
    this.evaluate(
      expected in this.value,
      `Expected object to contain key ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toContainValue(expected: any) {
    this.evaluate(
      Object.values(this.value).includes(expected),
      `Expected object to contain value ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toContainEntry(expected: any) {
    const [key, value] = expected;

    this.evaluate(
      this.value[key] === value,
      `Expected object to contain entry ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toContainEqualEntry(expected: any) {
    const [key, value] = expected;

    this.evaluate(
      this.value[key] == value,
      `Expected object to contain entry ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeGreaterThan(expected: any) {
    this.evaluate(
      this.value > expected,
      `Expected ${this.value} to be greater than ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeGreaterThanOrEqual(expected: any) {
    this.evaluate(
      this.value >= expected,
      `Expected ${this.value} to be greater than or equal to ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeLessThan(expected: any) {
    this.evaluate(
      this.value < expected,
      `Expected ${this.value} to be less than ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeLessThanOrEqual(expected: any) {
    this.evaluate(
      this.value <= expected,
      `Expected ${this.value} to be less than or equal to ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toBeCloseTo(expected: any, delta: number) {
    this.evaluate(
      Math.abs(this.value - expected) <= delta,
      `Expected ${this.value} to be close to ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }

  toHaveLengthOf(expected: any) {
    this.evaluate(
      this.value.length === expected,
      `Expected array to have length of ${expected}`,
      {
        expected,
        actual: this.value.length,
      }
    );
  }

  toHavePropertyOf(expected: any) {
    this.evaluate(
      this.value[expected],
      `Expected object to have property ${expected}`,
      {
        expected,
        actual: this.value,
      }
    );
  }
}

function test(description: string, fn: () => void) {
  try {
    fn();
    console.log(`✅ ${description}`);
  } catch (error: any) {
    if (error instanceof AssertionError) {
      console.log(
        `❌ ${description}\n   ${error.message}\n   ${error.getDiff(3)}`
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
