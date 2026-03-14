#!/bin/sh
set -eu

REPO="theuselessai/plit"
INSTALL_DIR="${PLIT_INSTALL_DIR:-$HOME/.local/bin}"

main() {
    os=$(uname -s | tr '[:upper:]' '[:lower:]')
    arch=$(uname -m)

    case "$os" in
        linux)  os_name="linux" ;;
        darwin) os_name="darwin" ;;
        *)      err "Unsupported OS: $os. Use 'cargo install plit' instead." ;;
    esac

    case "$arch" in
        x86_64|amd64)   arch_name="x86_64" ;;
        aarch64|arm64)  arch_name="aarch64" ;;
        i686|i386)      arch_name="i686" ;;
        *)              err "Unsupported architecture: $arch" ;;
    esac

    if [ "$os_name" = "darwin" ] && [ "$arch_name" = "i686" ]; then
        err "macOS i686 is not supported"
    fi

    artifact="plit-${os_name}-${arch_name}"
    archive="${artifact}.tar.gz"

    latest_tag=$(curl -sSf "https://api.github.com/repos/${REPO}/releases/latest" \
        | grep '"tag_name"' | head -1 | cut -d'"' -f4)

    if [ -z "$latest_tag" ]; then
        err "Could not determine latest release"
    fi

    url="https://github.com/${REPO}/releases/download/${latest_tag}/${archive}"

    printf "Installing plit %s (%s/%s)\n" "$latest_tag" "$os_name" "$arch_name"
    printf "  from: %s\n" "$url"
    printf "  to:   %s\n\n" "$INSTALL_DIR"

    mkdir -p "$INSTALL_DIR"

    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT

    curl -sSfL "$url" -o "$tmp/$archive" || err "Download failed. Check that ${archive} exists in release ${latest_tag}."
    tar xzf "$tmp/$archive" -C "$tmp"

    install -m 755 "$tmp/plit" "$INSTALL_DIR/plit"
    install -m 755 "$tmp/plit-gw" "$INSTALL_DIR/plit-gw"

    printf "\n  plit:    %s/plit\n" "$INSTALL_DIR"
    printf "  plit-gw: %s/plit-gw\n\n" "$INSTALL_DIR"

    case ":$PATH:" in
        *":$INSTALL_DIR:"*) ;;
        *)
            printf "Add to your PATH:\n\n"
            printf "  export PATH=\"%s:\$PATH\"\n\n" "$INSTALL_DIR"
            ;;
    esac

    printf "Run 'plit init' to get started.\n"
}

err() {
    printf "Error: %s\n" "$1" >&2
    exit 1
}

main
