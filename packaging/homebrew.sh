#!/bin/bash

set -xeuo pipefail

prefix="https://github.com/eugene-babichenko/fixit/releases/download/v$1/fixit-v$1"

function formula_platform() {
  cat <<EOF
      url "$prefix-$1.tar.gz"
      sha256 "$(wget -q -O - "$prefix-$1.sha256")"
EOF
}

cat > Formula/fixit.rb <<EOF
class Fixit < Formula
  desc "A utility to fix mistakes in your commands."
  homepage "https://github.com/eugene-babichenko/fixit"
  version "$1"

  on_macos do
    on_arm do
$(formula_platform "aarch64-apple-darwin")
    end
    on_intel do
$(formula_platform "x86_64-apple-darwin")
    end
  end
  on_linux do
    on_intel do
$(formula_platform "x86_64-unknown-linux-musl")
    end
  end

  def install
    bin.install "fixit"
  end
end
EOF
