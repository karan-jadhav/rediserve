#!/bin/sh

# From https://github.com/Homebrew/install/blob/master/install.sh
abort() {
  printf "%s\n" "$@"
  exit 1
}

# string formatters
if [ -t 1 ]; then
  tty_escape() { printf "\033[%sm" "$1"; }
else
  tty_escape() { :; }
fi
tty_mkbold() { tty_escape "1;$1"; }
tty_blue="$(tty_mkbold 34)"
tty_bold="$(tty_mkbold 39)"
tty_reset="$(tty_escape 0)"

ohai() {
  printf "${tty_blue}==>${tty_bold} %s${tty_reset}\n" "$1"
}

# End from https://github.com/Homebrew/install/blob/master/install.sh

download() {
  if command -v curl > /dev/null 2>&1; then
    curl -fsSL "$1"
  else
    wget -qO- "$1"
  fi
}

detect_platform() {
  local platform
  platform="$(uname -s | tr '[:upper:]' '[:lower:]')"

  case "${platform}" in
    linux) platform="linux" ;;
    darwin) platform="macos" ;;
    *) abort "Sorry, your platform is not supported." ;;
  esac

  printf '%s' "${platform}"
}

detect_arch() {
  local arch
  arch="$(uname -m)"

  case "${arch}" in
    x86_64) arch="x64" ;;
    aarch64) arch="arm64" ;;
    *) abort "Sorry, your architecture is not supported." ;;
  esac

  printf '%s' "${arch}"
}

download_and_install() {
  local platform arch archive_url tmp_dir binary_name="rediserve"
  platform="$(detect_platform)"
  arch="$(detect_arch)"
  version="latest" # This script always downloads the latest version; adjust as necessary.

  # Customize this URL based on your GitHub repository's structure
  archive_url="https://github.com/karan-jadhav/rediserve/releases/download/${version}/${binary_name}-${platform}-${arch}"

  tmp_dir="$(mktemp -d)" || abort "Tmpdir Error!"
  trap 'rm -rf "$tmp_dir"' EXIT INT TERM HUP

  ohai "Downloading ${binary_name} ${version} for ${platform}-${arch}"
  
  download "${archive_url}" > "${tmp_dir}/${binary_name}" || abort "Download Error!"
  chmod +x "${tmp_dir}/${binary_name}"

  # Customize the installation directory or process as necessary
  install_dir="/usr/local/bin"
  mv "${tmp_dir}/${binary_name}" "${install_dir}/${binary_name}" || abort "Installation Error!"
  ohai "${binary_name} installed successfully to ${install_dir}"
}

download_and_install || abort "Installation failed."
