type Hobby = 'reading' | 'writing' | 'coding' | 'swimming' | 'running';

enum Gender {
  Male = 'male',
  Female = 'female',
  Others = 'others',
}

interface Person {
  name: string;
  age: number;
  hobbies: Hobby[];
  gender: Gender;
}

const person = {
  name: 'John Doe',
  age: 30,
  hobbies: ['reading', 'coding'],
  gender: Gender.Male,
} satisfies Person;

console.log(person);
