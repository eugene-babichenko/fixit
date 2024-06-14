#!/bin/bash

set -xeuo pipefail

hashname="https://github.com/eugene-babichenko/fixit/releases/download/$1/fixit-$1"
version="$(echo "$1" | sed 's/v//' | sed 's/-/_/')"
sha256_x86_64="$(wget -q -O - "$hashname-x86_64-unknown-linux-musl.sha256")"
sha256_aarch64="$(wget -q -O - "$hashname-aarch64-unknown-linux-musl.sha256")"

cat > PKGBUILD <<EOF
# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit-bin
_pkgname="\${pkgname/-bin}"
pkgver=$version
_pkgver="\${pkgver//_/-}"
pkgrel=1
_repo="eugene-babichenko/\$_pkgname"
url="https://github.com/\$_repo"
pkgdesc="A utility to fix mistakes in your commands."
license=('MIT')
arch=('x86_64' 'aarch64')

_releases="\$url/releases/download/v\$_pkgver/\$_pkgname-v\$_pkgver"
_tree="https://raw.githubusercontent.com/\$_repo/v\$_pkgver"
_readme="\$_tree/README.md"
_license="\$_tree/LICENSE"
_linux="unknown-linux-musl"

source_x86_64=(
  "\$_pkgname-x86_64-\$_pkgver.tar.gz::\$_releases-x86_64-\$_linux.tar.gz"
  "\$_readme"
  "\$_license"
)
sha256sums_x86_64=(
  "$sha256_x86_64"
  'SKIP'
  'SKIP'
)

source_aarch64=(
  "\$_pkgname-aarch64-\$_pkgver.tar.gz::\$_releases-aarch64-\$_linux.tar.gz"
  "\$_readme"
  "\$_license"
)
sha256sums_aarch64=(
  "$sha256_aarch64"
  'SKIP'
  'SKIP'
)

package() {
  install -Dm755 "\$_pkgname" -t "\$pkgdir/usr/bin"
  install -Dm644 README.md -t "\$pkgdir/usr/share/doc/\$_pkgname"
  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/\$_pkgname"
}
EOF

makepkg --printsrcinfo > .SRCINFO

makepkg
