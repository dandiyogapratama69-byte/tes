// Loader Flipbook Timeline untuk ProCraft
// ProCraft load JS ini dulu, JS ini yang load .wasm

async function loadFlipbook(procrafter) {
  try {
    const wasmUrl = new URL("./flipbook_plugin.wasm", import.meta.url);
    const res = await fetch(wasmUrl);
    const buf = await res.arrayBuffer();
    const mod = await WebAssembly.instantiate(buf, {});
    const ex = mod.instance.exports;

    if (ex.flipbook_on_enable) ex.flipbook_on_enable();

    // Daftarin UI Flipbook ke ProCraft
    if (procrafter && procrafter.registerPanel) {
      procrafter.registerPanel({
        id: "flipbook-timeline",
        title: "Flipbook Timeline",
        // Tutup timeline bawaan, buka timeline flipbook
        onOpen: () => {
          if (procrafter.hideNativeTimeline) procrafter.hideNativeTimeline(true);
          return {
            fps: 24,
            native_timeline_hidden: true,
            tracks: [],
            note: "Layer != Frame. Pakai Flipbook Timeline."
          };
        }
      });
    }

    console.log("[Flipbook] Plugin loaded, wasm OK, tracks:", ex.flipbook_track_count ? ex.flipbook_track_count() : "unknown");
    return true;
  } catch (e) {
    console.error("[Flipbook] Gagal load wasm:", e);
    return false;
  }
}

// ProCraft manggil ini
if (typeof module !== "undefined") module.exports = { loadFlipbook };
if (typeof procraft !== "undefined") loadFlipbook(procraft);

export { loadFlipbook };
export default loadFlipbook;
