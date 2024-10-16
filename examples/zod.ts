// @ts-nocheck
import { z } from 'npm:zod';

const schema = z.object({
  name: z.string(),
  age: z.number(),
});

const data = schema.parse({ name: 'John Doe', age: 30 });

console.log(data);
