const rust = import('./pkg');

rust
  .then(m => m.run("todo_data"))
  .catch(console.error);