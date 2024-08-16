#![deny(warnings)]

use std::{fs::File, io::Write};

use clap::{Parser, Subcommand};
use sha2::{Digest, Sha256};
use xshell::{cmd, Shell};

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    Packaging {
        version: String,
        #[command(subcommand)]
        platform: Packaging,
    },
}

#[derive(Subcommand)]
enum Packaging {
    AurBin,
    Aur,
    Homebrew,
}

fn main() {
    let args = Args::parse();

    match args.cmd {
        Cmd::Packaging { version, platform } => match platform {
            Packaging::AurBin => aur_bin(version),
            Packaging::Aur => aur(version),
            Packaging::Homebrew => homebrew(version),
        },
    }
}

fn homebrew(version: String) {
    let aarch64_apple = homebrew_platform(&version, "aarch64-apple-darwin");
    let x86_64_apple = homebrew_platform(&version, "x86_64-apple-darwin");
    let linux = homebrew_platform(&version, "x86_64-unknown-linux-musl");

    println!(
        "class Fixit < Formula
  desc \"A utility to fix mistakes in your commands.\"
  homepage \"https://github.com/eugene-babichenko/fixit\"
  version \"{version}\"

  on_macos do
    on_arm do
      {aarch64_apple}
    end
    on_intel do
      {x86_64_apple}
    end
  end
  on_linux do
    on_intel do
      {linux}
    end
  end

  def install
    bin.install \"fixit\"
  end
end"
    );
}

fn homebrew_platform(version: &str, platform: &str) -> String {
    let (archive, hash) = bin_download_data(&version, platform);
    format!("url \"{archive}\"\n      sha256 \"{hash}\"")
}

fn aur(version: String) {
    let mut src = ureq::get(&format!(
        "https://github.com/eugene-babichenko/fixit/archive/v{version}.tar.gz"
    ))
    .call()
    .unwrap()
    .into_reader();
    let mut buf = Vec::new();
    src.read_to_end(&mut buf).unwrap();
    let mut hasher = Sha256::new();
    hasher.update(&buf);
    let checksum = hex::encode(hasher.finalize());

    let arch_version = arch_version(&version);
    let mut f = File::create("PKGBUILD").unwrap();
    writeln!(
        &mut f,
        "# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit
pkgver={arch_version}
pkgrel=1
url='https://github.com/eugene-babichenko/fixit'
pkgdesc='A utility to fix mistakes in your commands.'
license=('MIT')
arch=('x86_64' 'i686' 'aarch64' 'armv7h')
makedepends=('rust')
source=(\"$pkgname-$pkgver.tar.gz::$url/archive/v{version}.tar.gz\")
sha256sums=('{checksum}')

build() {{
  cd \"$pkgname-{version}\"
  export RUSTUP_TOOLCHAIN=stable
  export CARGO_TARGET_DIR=target
  cargo build --locked --release
}}

package() {{
  cd \"$pkgname-{version}\"
  install -Dm755 \"target/release/$pkgname\" -t \"$pkgdir/usr/bin\"
}}"
    )
    .unwrap();

    makepkg();
}

fn aur_bin(version: String) {
    let x86_64 = aur_bin_platform(&version, "x86_64");
    let aarch64 = aur_bin_platform(&version, "aarch64");

    let arch_version = arch_version(&version);

    let mut f = File::create("PKGBUILD").unwrap();
    writeln!(
        &mut f,
        "# Maintainer: Eugene Babichenko <eugene.babichenko@gmail.com>

pkgname=fixit-bin
pkgver={arch_version}
pkgrel=1
url='https://github.com/eugene-babichenko/fixit'
pkgdesc='A utility to fix mistakes in your commands.'
license=('MIT')
arch=('x86_64' 'aarch64')

{x86_64}

{aarch64}

package() {{
  install -Dm755 fixit -t \"$pkgdir/usr/bin\"
}}
"
    )
    .unwrap();

    makepkg();
}

fn arch_version(version: &str) -> String {
    version.replace('-', "_")
}

fn makepkg() {
    let sh = Shell::new().unwrap();
    let srcinfo = cmd!(sh, "makepkg --printsrcinfo").read().unwrap();
    sh.write_file(".SRCINFO", srcinfo).unwrap();
}

fn aur_bin_platform(version: &str, platform: &str) -> String {
    let dl_platform = format!("{platform}-unknown-linux-musl");
    let (archive, hash) = bin_download_data(&version, &dl_platform);
    format!(
        "source_{platform}=('fixit-{platform}-{version}.tar.gz::{archive}')
sha256sums_{platform}=('{hash}')"
    )
}

fn bin_download_data(version: &str, platform: &str) -> (String, String) {
    let prefix = format!("https://github.com/eugene-babichenko/fixit/releases/download/v{version}/fixit-v{version}-{platform}");
    let hash_url = format!("{prefix}.sha256");
    let hash = ureq::get(&hash_url).call().unwrap().into_string().unwrap();
    let archive = format!("{prefix}.tar.gz");
    (archive, hash)
}
