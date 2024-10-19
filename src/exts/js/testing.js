class AssertionError extends Error {
    constructor(message, data) {
        super(message);
        this.name = "AssertionError";
        this.data = data;
    }

    getDiff(indent = 0) {
        const expected = `\x1b[31m- ${this.data.expected}\x1b[0m`;
        const actual = `\x1b[32m+ ${this.data.actual}\x1b[0m`;
        const idn = " ".repeat(indent);

        return `${expected}\n${idn}${actual}`;
    }
}

class Assertion {
    constructor(value, expectation, invert = false) {
        this.value = value;
        this.expectation = expectation;
        this.invert = invert;
    }

    get not() {
        return new Assertion(this.value, this.expectation, !this.invert);
    }

    evaluate(condition, errorMessage, data) {
        if (this.invert ? condition : !condition) {
            throw new AssertionError(errorMessage, data);
        }
    }

    toBe(expected) {
        this.evaluate( this.value === expected, `Expected ${this.value} to be ${expected}`, {
            expected,
            actual: this.value,
        })
    }

    toEqual(expected) {
        if (this.invert ? this.value == expected : this.value != expected) {
            throw new AssertionError(`Expected ${this.value} to equal ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toStrictEqual(expected) {
        if (this.invert ? this.value === expected : this.value !== expected) {
            throw new AssertionError(`Expected ${this.value} to strictly equal ${expected}`, {
                expected,
                actual: this.value,
            });
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

    toBeInstanceOf(expected) {
        if (this.invert ? this.value instanceof expected : !(this.value instanceof expected)) {
            throw new AssertionError(`Expected ${this.value} to be an instance of ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toMatch(expected) {
        if (this.invert ? this.value.match(expected) : !this.value.match(expected)) {
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

    toThrowError(expected) {
        let error = null;

        try {
            this.value();
        } catch (e) {
            error = e;
        }

        if (this.invert ? error instanceof expected : !(error instanceof expected)) {
            throw new AssertionError(`Expected function to throw an instance of ${expected}`, {
                expected,
                actual: error,
            });
        }
    }

    toHaveProperty(expected) {
        if (this.invert ? this.value[expected] : !this.value[expected]) {
            throw new AssertionError(`Expected object to have property ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toHaveLength(expected) {
        if (this.invert ? this.value.length === expected : this.value.length !== expected) {
            throw new AssertionError(`Expected array to have length of ${expected}`, {
                expected,
                actual: this.value.length,
            });
        }
    }

    toContain(expected) {
        if (this.invert ? this.value.includes(expected) : !this.value.includes(expected)) {
            throw new AssertionError(`Expected array to contain ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toContainEqual(expected) {
        if (this.invert ? this.value.includes(expected) : !this.value.includes(expected)) {
            throw new AssertionError(`Expected array to contain ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toContainKey(expected) {
        if (this.invert ? expected in this.value : !(expected in this.value)) {
            throw new AssertionError(`Expected object to contain key ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toContainValue(expected) {
        if (this.invert ? Object.values(this.value).includes(expected) : !Object.values(this.value).includes(expected)) {
            throw new AssertionError(`Expected object to contain value ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toContainEntry(expected) {
        const [key, value] = expected;

        if (this.invert ? this.value[key] === value : this.value[key] !== value) {
            throw new AssertionError(`Expected object to contain entry ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toContainEqualEntry(expected) {
        const [key, value] = expected;

        if (this.invert ? this.value[key] == value : this.value[key] != value) {
            throw new AssertionError(`Expected object to contain entry ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toBeGreaterThan(expected) {
        if (this.invert ? this.value > expected : this.value <= expected) {
            throw new AssertionError(`Expected ${this.value} to be greater than ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toBeGreaterThanOrEqual(expected) {
        if (this.invert ? this.value >= expected : this.value < expected) {
            throw new AssertionError(`Expected ${this.value} to be greater than or equal to ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toBeLessThan(expected) {
        if (this.invert ? this.value < expected : this.value >= expected) {
            throw new AssertionError(`Expected ${this.value} to be less than ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toBeLessThanOrEqual(expected) {
        if (this.invert ? this.value <= expected : this.value > expected) {
            throw new AssertionError(`Expected ${this.value} to be less than or equal to ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toBeCloseTo(expected, delta) {
        if (this.invert ? Math.abs(this.value - expected) <= delta : Math.abs(this.value - expected) > delta) {
            throw new AssertionError(`Expected ${this.value} to be close to ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }

    toHaveLengthOf(expected) {
        if (this.invert ? this.value.length === expected : this.value.length !== expected) {
            throw new AssertionError(`Expected array to have length of ${expected}`, {
                expected,
                actual: this.value.length,
            });
        }
    }

    toHavePropertyOf(expected) {
        if (this.invert ? this.value[expected] : !this.value[expected]) {
            throw new AssertionError(`Expected object to have property ${expected}`, {
                expected,
                actual: this.value,
            });
        }
    }
}

function test(description, fn) {
    try {
        fn();
        console.log(`✅ ${description}`);
    } catch (error) {
        if (error instanceof AssertionError) {
            console.log(`❌ ${description}\n   ${error.message}\n   ${error.getDiff(3)}`);
        } else {
            console.log(`❌ ${description}\n  ${error.stack}`);
        }
    }
}

function expect(value) {
    return new Assertion(value);
}

Object.assign(globalThis, {
    test,
    it: test,
    expect,
    Assertion,
    AssertionError,
});