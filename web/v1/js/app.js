import { buildEncoders, utf8ByteLength } from "./encoders.js";

const encoders = buildEncoders();
const encoderNames = Object.keys(encoders);

const $input = document.getElementById("input");
const $output = document.getElementById("output");
const $encoder = document.getElementById("encoder");
const $status = document.getElementById("status");
const $inBytes = document.getElementById("in-bytes");
const $outBytes = document.getElementById("out-bytes");
const $btnEncode = document.getElementById("btn-encode");
const $btnDecode = document.getElementById("btn-decode");
const $btnCopy = document.getElementById("btn-copy");
const $btnSwap = document.getElementById("btn-swap");

for (const name of encoderNames) {
  const opt = document.createElement("option");
  opt.value = name;
  opt.textContent = name;
  $encoder.appendChild(opt);
}

function setStatus(msg, isError = false) {
  $status.textContent = msg;
  $status.className = isError ? "status error" : "status";
}

function updateByteLabels() {
  $inBytes.textContent = String(utf8ByteLength($input.value));
  $outBytes.textContent = String(utf8ByteLength($output.value));
}

$input.addEventListener("input", updateByteLabels);
$output.addEventListener("input", updateByteLabels);
updateByteLabels();

function getEncoder() {
  return encoders[$encoder.value];
}

$btnEncode.addEventListener("click", () => {
  setStatus("");
  try {
    const enc = getEncoder();
    const out = enc.encode($input.value);
    $output.value = out;
    updateByteLabels();
    setStatus("Encoded.");
  } catch (e) {
    setStatus(e.message || String(e), true);
  }
});

$btnDecode.addEventListener("click", () => {
  setStatus("");
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

$btnSwap.addEventListener("click", () => {
  const a = $input.value;
  $input.value = $output.value;
  $output.value = a;
  updateByteLabels();
  setStatus("Swapped fields.");
});
