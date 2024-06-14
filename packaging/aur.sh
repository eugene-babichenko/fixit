#!/bin/bash
repo="https://github.com/eugene-babichenko/fixit"

cat > PKGBUILD <<EOF
# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit
pkgver=$(echo $1 | sed 's/v//' | sed 's/-/_/')
_pkgver="\${pkgver//_/-}"
pkgrel=1
url="$repo"
pkgdesc="A utility to fix mistakes in your commands."
license=('MIT')
arch=('x86_64' 'i686' 'aarch64' 'armv7h')
makedepends=('rust')
source=("\$pkgname-\$pkgver.tar.gz::\$url/archive/v\$_pkgver.tar.gz")
sha256sums=("$(wget -q -O- "$repo/archive/$1.tar.gz" | shasum -a 256 | cut -d ' ' -f 1 | tr -d '\n')")

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
