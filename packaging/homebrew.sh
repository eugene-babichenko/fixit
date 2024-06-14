#!/bin/bash

prefix="https://github.com/eugene-babichenko/fixit/releases/download/$1/fixit-$1"

cat > Formula/fixit.rb <<EOF
class Fixit < Formula
  desc "A utility to fix mistakes in your commands."
  homepage "https://github.com/eugene-babichenko/fixit"
  version "$1"

  on_macos do
    on_arm do
      url "$prefix-aarch64-apple-darwin.tar.gz"
      sha256 "$(wget -O - $prefix-aarch64-apple-darwin.sha256)"
    end
    on_intel do
      url "$prefix-x86_64-apple-darwin.tar.gz"
      sha256 "$(wget -O - $prefix-x86_64-apple-darwin.sha256)"
    end
  end
  on_linux do
    on_intel do
      url "$prefix-x86_64-unknown-linux-musl.tar.gz"
      sha256 "$(wget -O - $prefix-x86_64-unknown-linux-musl.sha256)"
    end
  end

  def install
    bin.install "fixit"
  end
end
EOF
