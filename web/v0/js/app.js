import { buildEncodersFromFixtures, utf8ByteLength } from "./encoders.js";

/** UTF-8 byte limits on the encoded string you paste/send (not raw plaintext). */
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

/** @type {Awaited<ReturnType<typeof buildEncodersFromFixtures>> | null} */
let encoders = null;

function setStatus(msg, isError = false) {
  $status.textContent = msg;
  $status.className = isError ? "status error" : "status";
}

function clearTransportWarnings() {
  $transportWarn.classList.remove("visible");
  $transportWarn.textContent = "";
}

/**
 * @param {number | null} payloadUtf8Bytes — UTF-8 length of encoded output; null hides panel
 */
function setTransportWarnings(payloadUtf8Bytes) {
  if (payloadUtf8Bytes == null || payloadUtf8Bytes <= LIMIT_MESHCORE_PAYLOAD_UTF8) {
    clearTransportWarnings();
    return;
  }
  const parts = [];
  if (payloadUtf8Bytes > LIMIT_MESHCORE_PAYLOAD_UTF8) {
    parts.push(
      `Encoded output is <strong>${payloadUtf8Bytes} B</strong> — with <strong>Meshcore</strong> it may be truncated (limit: <strong>~${LIMIT_MESHCORE_PAYLOAD_UTF8} B</strong>)`,
    );
  }
  if (payloadUtf8Bytes > LIMIT_MESHTASTIC_PAYLOAD_UTF8) {
    parts.push(
      `Encoded output is <strong>${payloadUtf8Bytes} B</strong> — with <strong>Meshtastic</strong> it may not fit a single packet (limit: <strong>~${LIMIT_MESHTASTIC_PAYLOAD_UTF8} B</strong>)`,
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

function getEncoder() {
  if (!encoders) {
    throw new Error("Encoders not loaded yet.");
  }
  return encoders[$encoder.value];
}

async function main() {
  setStatus("Loading dictionaries…");
  try {
    encoders = await buildEncodersFromFixtures();
  } catch (e) {
    setStatus(
      (e && e.message) || String(e),
      true,
    );
    return;
  }

  const encoderNames = Object.keys(encoders);
  for (const name of encoderNames) {
    const opt = document.createElement("option");
    opt.value = name;
    opt.textContent = name;
    $encoder.appendChild(opt);
  }

  $input.addEventListener("input", updateByteLabels);
  $output.addEventListener("input", updateByteLabels);
  updateByteLabels();

  $btnEncode.addEventListener("click", () => {
    setStatus("");
    clearTransportWarnings();
    try {
      const enc = getEncoder();
      const out = enc.encode($input.value);
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
      const enc = getEncoder();
      const text = enc.decode($input.value);
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

  setStatus("Ready.");
}

main();
