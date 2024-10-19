let i = 1;

const interval = setInterval(() => {
  console.log(`Hello ${i}`);

  if (i++ === 10) {
    clearInterval(interval);
    console.log("Done!");
  }
}, 1000);
