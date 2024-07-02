#!/bin/bash

set -xeuo pipefail

pkgname='fixit'
version="$1"
repo="eugene-babichenko/$pkgname"
url="https://github.com/$repo"
hashname="$url/releases/download/v$version/fixit-v$version"
releases="$url/releases/download/v$version/$pkgname-v$version"
tree="https://raw.githubusercontent.com/$repo/v$version"
readme="$tree/README.md"
license="$tree/LICENSE"

function pkgbuild_platform() {
  suffix='unknown-linux-musl'
  cat <<EOF
source_$1=(
  '$pkgname-$1-$version.tar.gz::$releases-$1-$suffix.tar.gz'
  '$readme'
  '$license'
)
sha256sums_$1=(
  '$(wget -q -O - "$hashname-$1-$suffix.sha256")'
  'SKIP'
  'SKIP'
)
EOF
}

cat > PKGBUILD <<EOF
# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=$pkgname-bin
pkgver=${version/-/_}
pkgrel=1
url='$url'
pkgdesc='A utility to fix mistakes in your commands.'
license=('MIT')
arch=('x86_64' 'aarch64')

$(pkgbuild_platform "x86_64")

$(pkgbuild_platform "aarch64")

package() {
  install -Dm755 $pkgname -t "\$pkgdir/usr/bin"
  install -Dm644 README.md -t "\$pkgdir/usr/share/doc/$pkgname"
  install -Dm644 LICENSE -t "\$pkgdir/usr/share/licenses/$pkgname"
}
EOF

makepkg --printsrcinfo > .SRCINFO

makepkg
