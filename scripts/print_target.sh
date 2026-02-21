#!/usr/bin/env bash
set -euo pipefail

os="$(uname -s)"
arch="$(uname -m)"

case "${arch}" in
  x86_64|amd64)
    arch="x86_64"
    ;;
  aarch64|arm64)
    arch="aarch64"
    ;;
  *)
    echo "Unsupported architecture: ${arch}" >&2
    exit 1
    ;;
esac

case "${os}" in
  Linux)
    os_suffix="unknown-linux-gnu"
    ;;
  Darwin)
    os_suffix="apple-darwin"
    ;;
  MINGW*|MSYS*|CYGWIN*)
    os_suffix="pc-windows-msvc"
    ;;
  *)
    echo "Unsupported operating system: ${os}" >&2
    exit 1
    ;;
esac

echo "${arch}-${os_suffix}"
