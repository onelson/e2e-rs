// cache for the module load promise
let wasmModule = null;

export default async function getClient(prefix) {
    wasmModule = wasmModule || import("e2e-client");
    const { MessagesAPI } = await wasmModule;
    return new MessagesAPI(prefix);
};