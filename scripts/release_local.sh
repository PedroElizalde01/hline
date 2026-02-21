#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"

BIN_NAME="${BIN_NAME:-hline}"
TARGET_TRIPLE="${TARGET_TRIPLE:-$("${SCRIPT_DIR}/print_target.sh")}"
ARCH="${TARGET_TRIPLE%%-*}"
TARGET_SUFFIX="${TARGET_TRIPLE#*-}"
ARTIFACT_BASENAME="${BIN_NAME}-${ARCH}-${TARGET_SUFFIX}"
DIST_DIR="${ROOT_DIR}/dist"
STAGE_DIR="${DIST_DIR}/${ARTIFACT_BASENAME}"
ARCHIVE_PATH="${DIST_DIR}/${ARTIFACT_BASENAME}.tar.gz"
CHECKSUM_PATH="${ARCHIVE_PATH}.sha256"

mkdir -p "${DIST_DIR}"
rm -rf "${STAGE_DIR}" "${ARCHIVE_PATH}" "${CHECKSUM_PATH}"

echo "Building ${BIN_NAME} for ${TARGET_TRIPLE}..."
cargo build --release --target "${TARGET_TRIPLE}" --manifest-path "${ROOT_DIR}/Cargo.toml"

BIN_PATH="${ROOT_DIR}/target/${TARGET_TRIPLE}/release/${BIN_NAME}"
if [[ ! -x "${BIN_PATH}" ]]; then
  echo "Built binary not found: ${BIN_PATH}" >&2
  exit 1
fi

mkdir -p "${STAGE_DIR}"
cp "${BIN_PATH}" "${STAGE_DIR}/${BIN_NAME}"
cp "${ROOT_DIR}/README.md" "${STAGE_DIR}/README.md"
cp "${ROOT_DIR}/LICENSE" "${STAGE_DIR}/LICENSE"

tar -C "${DIST_DIR}" -czf "${ARCHIVE_PATH}" "${ARTIFACT_BASENAME}"

if command -v sha256sum >/dev/null 2>&1; then
  sha256sum "${ARCHIVE_PATH}" | tee "${CHECKSUM_PATH}"
elif command -v shasum >/dev/null 2>&1; then
  shasum -a 256 "${ARCHIVE_PATH}" | tee "${CHECKSUM_PATH}"
else
  echo "Warning: sha256 tool not available; checksum file not generated." >&2
fi

echo
echo "Release artifact created:"
echo "  ${ARCHIVE_PATH}"
if [[ -f "${CHECKSUM_PATH}" ]]; then
  echo "  ${CHECKSUM_PATH}"
fi
