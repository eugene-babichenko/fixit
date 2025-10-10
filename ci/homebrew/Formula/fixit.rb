class Fixit < Formula
  desc "A utility to fix mistakes in your commands."
  homepage "https://github.com/eugene-babichenko/fixit"
  version "__VERSION__"

  on_macos do
    on_arm do
      url "https://github.com/eugene-babichenko/fixit/releases/download/v__VERSION__/fixit-v__VERSION__-aarch64-apple-darwin.tar.gz"
      sha256 "__SHA256_aarch64-apple-darwin__"
    end

    on_intel do
      url "https://github.com/eugene-babichenko/fixit/releases/download/v__VERSION__/fixit-v__VERSION__-x86_64-apple-darwin.tar.gz"
      sha256 "__SHA256_x86_64-apple-darwin__"
    end
  end

  def install
    bin.install "fixit"
  end
end
