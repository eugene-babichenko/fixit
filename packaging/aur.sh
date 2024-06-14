#!/bin/bash
repo="https://github.com/eugene-babichenko/fixit"

cat > PKGBUILD <<EOF
# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit
pkgver=$(echo $1 | sed 's/v//' | sed 's/-/_/')
pkgrel=1
url="$repo"
pkgdesc="A utility to fix mistakes in your commands."
license=('MIT')
arch=('x86_64' 'aarch64')
makedepends=('rust')
source=("\$pkgname-\$pkgver.tar.gz::\$url/archive/v\${pkgver//_/-}.tar.gz")
sha256sums=("$(wget -O - $repo/archive/$1.tar.gz | shasum -a 256 | cut -d ' ' -f 1 | tr -d '\n')")

build() {
  cd "\$pkgname-\${pkgver//_/-}"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --frozen --release
}

package() {
  cd "\$pkgname-\${pkgver//_/-}"
  install -Dm755 "target/release/\$pkgname" -t "\$pkgdir/usr/bin"
  install -Dm644 README.md -t "\$pkgdir/usr/share/doc/\$pkgname"
  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/\$pkgname"
}
EOF

makepkg --printsrcinfo > .SRCINFO
