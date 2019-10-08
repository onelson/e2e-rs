// cache for the module load promise
let _wasmModule = null;

async function getMod() {
  _wasmModule = _wasmModule || import("e2e-client");
  return _wasmModule;
}

export async function getClient() {
  const { MessagesAPI } = await getMod();
  return new MessagesAPI("/api");
}
