#!/bin/bash

set -xeuo pipefail

repo="https://github.com/eugene-babichenko/fixit"
version="$(echo "$1" | sed 's/v//' | sed 's/-/_/')"
checksum="$(wget -q -O- "$repo/archive/$1.tar.gz" | sha256sum | cut -d ' ' -f 1 | tr -d '\n')"

cat > PKGBUILD <<EOF
# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit
pkgver=$version
_pkgver="\${pkgver//_/-}"
pkgrel=1
url="$repo"
pkgdesc="A utility to fix mistakes in your commands."
license=('MIT')
arch=('x86_64' 'i686' 'aarch64' 'armv7h')
makedepends=('rust')
source=("\$pkgname-\$pkgver.tar.gz::\$url/archive/v\$_pkgver.tar.gz")
sha256sums=('$checksum')

build() {
  cd "\$pkgname-\$_pkgver"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --locked --release
}

package() {
  cd "\$pkgname-\$_pkgver"
  install -Dm755 "target/release/\$pkgname" -t "\$pkgdir/usr/bin"
  install -Dm644 README.md -t "\$pkgdir/usr/share/doc/\$pkgname"
  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/\$pkgname"
}
EOF

makepkg --printsrcinfo > .SRCINFO

makepkg
