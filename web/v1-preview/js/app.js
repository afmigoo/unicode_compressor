const LIMIT_MESHCORE_PAYLOAD_UTF8 = 150;
const LIMIT_MESHTASTIC_PAYLOAD_UTF8 = 200;

const $input = document.getElementById("input");
const $output = document.getElementById("output");
const $encoder = document.getElementById("encoder");
const $status = document.getElementById("status");
const $inBytes = document.getElementById("in-bytes");
const $outBytes = document.getElementById("out-bytes");
const $btnEncode = document.getElementById("btn-encode");
const $btnDecode = document.getElementById("btn-decode");
const $btnCopy = document.getElementById("btn-copy");
const $btnClear = document.getElementById("btn-clear");
const $transportWarn = document.getElementById("transport-warn");

let wasmEncode = null;
let wasmDecode = null;
let wasmListEncoders = null;
const WASM_INIT_TIMEOUT_MS = 60000;
const CONTROLS = [$encoder, $input, $output, $btnEncode, $btnDecode, $btnCopy, $btnClear];

function utf8ByteLength(text) {
  return new TextEncoder().encode(text).length;
}

function setStatus(msg, isError = false) {
  $status.textContent = msg;
  $status.className = isError ? "status error" : "status";
}

function clearTransportWarnings() {
  $transportWarn.classList.remove("visible");
  $transportWarn.textContent = "";
}

function setTransportWarnings(payloadUtf8Bytes) {
  if (payloadUtf8Bytes == null || payloadUtf8Bytes <= LIMIT_MESHCORE_PAYLOAD_UTF8) {
    clearTransportWarnings();
    return;
  }
  const parts = [];
  if (payloadUtf8Bytes > LIMIT_MESHCORE_PAYLOAD_UTF8) {
    parts.push(
      `Encoded output is <strong>${payloadUtf8Bytes} B</strong> - with <strong>Meshcore</strong> it may be truncated (limit: <strong>~${LIMIT_MESHCORE_PAYLOAD_UTF8} B</strong>)`,
    );
  }
  if (payloadUtf8Bytes > LIMIT_MESHTASTIC_PAYLOAD_UTF8) {
    parts.push(
      `Encoded output is <strong>${payloadUtf8Bytes} B</strong> - with <strong>Meshtastic</strong> it may not fit a single packet (limit: <strong>~${LIMIT_MESHTASTIC_PAYLOAD_UTF8} B</strong>)`,
    );
  }
  $transportWarn.innerHTML =
    "<p style=\"margin:0 0 0.25rem\">Warning:</p><ul>" +
    parts.map((p) => `<li>${p}</li>`).join("") +
    "</ul>";
  $transportWarn.classList.add("visible");
}

function updateByteLabels() {
  $inBytes.textContent = String(utf8ByteLength($input.value));
  $outBytes.textContent = String(utf8ByteLength($output.value));
}

function getEncoderName() {
  return $encoder.value;
}

function ensureWasmLoaded() {
  if (!wasmEncode || !wasmDecode || !wasmListEncoders) {
    throw new Error("WASM module is not ready yet.");
  }
}

function setControlsEnabled(enabled) {
  for (const el of CONTROLS) {
    el.disabled = !enabled;
  }
}

async function withTimeout(promise, timeoutMs, label) {
  let timeoutId = null;
  const timeoutPromise = new Promise((_, reject) => {
    timeoutId = setTimeout(() => {
      reject(new Error(`${label} timed out after ${timeoutMs} ms`));
    }, timeoutMs);
  });
  try {
    return await Promise.race([promise, timeoutPromise]);
  } finally {
    clearTimeout(timeoutId);
  }
}

async function main() {
  setStatus("Loading wasm module JS...");
  try {
    const wasm = await import("../pkg/unipress.js");
    setStatus("Loading wasm binary...");
    const wasmUrl = new URL("../pkg/unipress_bg.wasm", import.meta.url);
    const wasmResp = await withTimeout(
      fetch(wasmUrl),
      WASM_INIT_TIMEOUT_MS,
      "WASM download",
    );
    if (!wasmResp.ok) {
      throw new Error(`Failed to load wasm binary: HTTP ${wasmResp.status}`);
    }
    setStatus("Reading wasm binary...");
    const wasmBytes = await withTimeout(
      wasmResp.arrayBuffer(),
      WASM_INIT_TIMEOUT_MS,
      "WASM read",
    );
    setStatus("Initializing wasm module...");
    await withTimeout(
      wasm.default(wasmBytes),
      WASM_INIT_TIMEOUT_MS,
      "WASM initialization",
    );
    wasmEncode = wasm.encode;
    wasmDecode = wasm.decode;
    wasmListEncoders = wasm.list_encoders;
  } catch (e) {
    setStatus((e && e.message) || String(e), true);
    return;
  }

  const encoderNames = wasmListEncoders();
  for (const name of encoderNames) {
    const opt = document.createElement("option");
    opt.value = name;
    opt.textContent = name;
    $encoder.appendChild(opt);
  }
  if (encoderNames.includes("bpe_meshcoretel_ru")) {
    $encoder.value = "bpe_meshcoretel_ru";
  } else if (encoderNames.length > 0) {
    $encoder.value = encoderNames[0];
  }

  $input.addEventListener("input", updateByteLabels);
  $output.addEventListener("input", updateByteLabels);
  updateByteLabels();

  $btnEncode.addEventListener("click", () => {
    setStatus("");
    clearTransportWarnings();
    try {
      ensureWasmLoaded();
      const plain = $input.value;
      const out = wasmEncode(plain, getEncoderName());
      $output.value = out;
      updateByteLabels();
      setTransportWarnings(utf8ByteLength($output.value));
      setStatus("Encoded.");
    } catch (e) {
      setStatus(e.message || String(e), true);
    }
  });

  $btnDecode.addEventListener("click", () => {
    setStatus("");
    clearTransportWarnings();
    try {
      ensureWasmLoaded();
      const text = wasmDecode($input.value, getEncoderName());
      $output.value = text;
      updateByteLabels();
      setStatus("Decoded.");
    } catch (e) {
      setStatus(e.message || String(e), true);
    }
  });

  $btnCopy.addEventListener("click", async () => {
    try {
      await navigator.clipboard.writeText($output.value);
      setStatus("Copied output.");
    } catch (e) {
      setStatus("Copy failed: " + (e.message || String(e)), true);
    }
  });

  $btnClear.addEventListener("click", () => {
    $input.value = "";
    $output.value = "";
    updateByteLabels();
    clearTransportWarnings();
    setStatus("");
  });

  setControlsEnabled(true);
  setStatus("Ready.");
}

main();
