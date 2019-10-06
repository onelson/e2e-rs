const rust = import("./pkg/e2e_client");

rust
  .then(m => {
    console.info("Loaded wasm module", m);
    return m;
  })
  .catch(e => console.error(e));
