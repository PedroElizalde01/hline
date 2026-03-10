#!/usr/bin/env bash
set -euo pipefail

APP_NAME="hline"
REPO="${REPO:-PedroElizalde01/hline}"
VERSION="${VERSION:-}"
BIN_DIR="${BIN_DIR:-$HOME/.local/bin}"
HLINE_BASE_URL="${HLINE_BASE_URL:-}"

usage() {
  cat <<'EOF'
Install hline from GitHub Releases.

Usage:
  install.sh [--repo OWNER/REPO] [--version vX.Y.Z] [--bin-dir PATH]

Options:
  -r, --repo      GitHub repo in OWNER/REPO format
  -v, --version   Release tag (default: latest release tag)
  -b, --bin-dir   Install directory (default: ~/.local/bin)
  -h, --help      Show this help

Environment overrides:
  REPO, VERSION, BIN_DIR, HLINE_BASE_URL

Local test example:
  HLINE_BASE_URL="file:///absolute/path/to/dist" ./install.sh --version v0.1.0
EOF
}

detect_target() {
  local os arch
  os="$(uname -s)"
  arch="$(uname -m)"

  case "${arch}" in
    x86_64|amd64) arch="x86_64" ;;
    aarch64|arm64) arch="aarch64" ;;
    *)
      echo "Unsupported architecture: ${arch}" >&2
      return 1
      ;;
  esac

  case "${os}" in
    Linux) echo "${arch}-unknown-linux-gnu" ;;
    Darwin) echo "${arch}-apple-darwin" ;;
    MINGW*|MSYS*|CYGWIN*) echo "${arch}-pc-windows-msvc" ;;
    *)
      echo "Unsupported OS: ${os}" >&2
      return 1
      ;;
  esac
}

resolve_latest_version() {
  local api_url
  api_url="https://api.github.com/repos/${REPO}/releases/latest"
  curl -fsSL "${api_url}" | sed -n 's/.*"tag_name":[[:space:]]*"\([^"]*\)".*/\1/p' | head -n 1
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    -r|--repo)
      REPO="$2"
      shift 2
      ;;
    -v|--version)
      VERSION="$2"
      shift 2
      ;;
    -b|--bin-dir)
      BIN_DIR="$2"
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage >&2
      exit 1
      ;;
  esac
done

if [[ -z "${VERSION}" ]]; then
  VERSION="$(resolve_latest_version || true)"
  if [[ -z "${VERSION}" ]]; then
    echo "Could not resolve latest release tag from GitHub API for ${REPO}." >&2
    echo "Use --version <tag> after your first release exists." >&2
    exit 1
  fi
fi

TARGET_TRIPLE="$(detect_target)"
ARCH="${TARGET_TRIPLE%%-*}"
TARGET_SUFFIX="${TARGET_TRIPLE#*-}"
ASSET_BASENAME="${APP_NAME}-${ARCH}-${TARGET_SUFFIX}"
ARCHIVE_NAME="${ASSET_BASENAME}.tar.gz"
CHECKSUM_NAME="${ARCHIVE_NAME}.sha256"
if [[ -n "${HLINE_BASE_URL}" ]]; then
  BASE_URL="${HLINE_BASE_URL%/}"
else
  BASE_URL="https://github.com/${REPO}/releases/download/${VERSION}"
fi
ARCHIVE_URL="${BASE_URL}/${ARCHIVE_NAME}"
CHECKSUM_URL="${BASE_URL}/${CHECKSUM_NAME}"

TMP_DIR="$(mktemp -d)"
cleanup() { rm -rf "${TMP_DIR}"; }
trap cleanup EXIT

mkdir -p "${BIN_DIR}"

echo "Downloading ${ARCHIVE_URL}"
curl -fsSL -o "${TMP_DIR}/${ARCHIVE_NAME}" "${ARCHIVE_URL}"

if curl -fsSL -o "${TMP_DIR}/${CHECKSUM_NAME}" "${CHECKSUM_URL}"; then
  if command -v sha256sum >/dev/null 2>&1; then
    expected="$(cut -d ' ' -f 1 "${TMP_DIR}/${CHECKSUM_NAME}")"
    actual="$(sha256sum "${TMP_DIR}/${ARCHIVE_NAME}" | cut -d ' ' -f 1)"
    if [[ "${expected}" != "${actual}" ]]; then
      echo "Checksum mismatch for ${ARCHIVE_NAME}" >&2
      exit 1
    fi
    echo "Checksum verified."
  else
    echo "sha256sum not found; skipping checksum verification."
  fi
else
  echo "Checksum file missing; continuing without checksum verification."
fi

tar -xzf "${TMP_DIR}/${ARCHIVE_NAME}" -C "${TMP_DIR}"
install -m 0755 "${TMP_DIR}/${ASSET_BASENAME}/${APP_NAME}" "${BIN_DIR}/${APP_NAME}"

echo
echo "Installed ${APP_NAME} ${VERSION} to ${BIN_DIR}/${APP_NAME}"
case ":$PATH:" in
  *":${BIN_DIR}:"*) ;;
  *)
    echo "Note: ${BIN_DIR} is not in PATH for this shell."
    echo "Add this to your shell profile:"
    echo "  export PATH=\"${BIN_DIR}:\$PATH\""
    ;;
esac
