#!/bin/bash

# This script is meant to run on macOS. Running it on other targets maybe successful, but the success is not guaranteed.

set -xeuo pipefail

# this has to do with cargo zigbuild on macos
ulimit -n 8192

tag="$(git describe --tags --abbrev=0)"
version="${tag#v}"

cargo_version="$(cargo metadata --format-version 1 | jq -j --raw-output '.packages[] | select(.name == "fixit-cli") | .version')"
if [ "$cargo_version" != "$version" ]; then
     echo "Version mismatch: Cargo.toml ($cargo_version) vs Git tag ($tag)"
     exit 1
fi

rm -rf ./artifacts
mkdir -p ./artifacts

awk '/^## /{c++} c==2{print} c==3{exit}' CHANGELOG.md | sed 's/^ *//;s/ *$//' > ./artifacts/notes.md
cat ./artifacts/notes.md

git clone ssh://aur@aur.archlinux.org/fixit-bin ./artifacts/aur
git clone git@github.com:eugene-babichenko/homebrew-fixit.git ./artifacts/homebrew
git clone -b gh-pages git@github.com:eugene-babichenko/fixit.git ./artifacts/repos

init_pkg_file () {
    cp "./ci/$1" "./artifacts/$1"
    sed -i "s/__VERSION__/$version/g" "./artifacts/$1"
}

init_pkg_file aur/PKGBUILD
init_pkg_file aur/.SRCINFO
init_pkg_file homebrew/Formula/fixit.rb

linux_targets=(aarch64-unknown-linux-musl x86_64-unknown-linux-musl)
apple_targets=(x86_64-apple-darwin aarch64-apple-darwin)
targets=("${linux_targets[@]}" "${apple_targets[@]}")

for target in "${targets[@]}"; do
    rustup target add "$target"
    cargo zigbuild --release --locked --target "$target"
    tar -czvf "./artifacts/fixit-$tag-$target.tar.gz" -C "./target/$target/release" "fixit"
done

make_shasum() {
    shasum="$(shasum -a 256 "./artifacts/fixit-$tag-$target.tar.gz" | cut -d ' ' -f 1 | tr -d '\n')"
}

set_pkg_shasum() {
    sed -i "s/__SHA256_${target}__/$shasum/g" "./artifacts/$1"
}

for target in "${linux_targets[@]}"; do
    cargo deb --no-build --no-strip --profile release --output ./artifacts/repos/ppa --target "$target"
    cargo generate-rpm --auto-req no --profile release --output ./artifacts/repos/rpm --target "$target"
    make_shasum
    set_pkg_shasum aur/PKGBUILD
    set_pkg_shasum aur/.SRCINFO
done

for target in "${apple_targets[@]}"; do
    make_shasum
    set_pkg_shasum homebrew/Formula/fixit.rb
done

gh release create "$tag" --notes-file ./artifacts/notes.md
# TODO upload deb and rpm
gh release upload "$tag" ./artifacts/*.tag.gz

cd ./artifacts/repos/ppa
dpkg-scanpackages --multiversion . /dev/null | gzip -9c > Packages.gz

cd ../rpm
docker run -v "$PWD:/repo" fedora /bin/sh -c "dnf -y install createrepo && cd /repo && createrepo ."

cd ../../homebrew
# TODO push all repos

cd ../..

cargo publish
