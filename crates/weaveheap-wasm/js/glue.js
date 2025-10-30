export async function init(url) {
  const resp = await fetch(url);
  const bytes = await resp.arrayBuffer();
  const { instance } = await WebAssembly.instantiate(bytes, {});
  return {
    versionMajor: () => instance.exports.weaveheap_wasm_version_major?.() ?? 0,
  };
}
