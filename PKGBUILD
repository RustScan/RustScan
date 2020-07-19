# Maintainer: Bee <brandon@skerritt.blog>
pkgname=rustscan-bin
pkgver=1.0.1
pkgrel=1
pkgdesc="Faster Nmap Scanning with Rust"
url="https://github.com/brandonskerritt/rustscan"
license=("MIT")
arch=("x86_64")
provides=("rustscan")
options=("strip")
source=("https://github.com/brandonskerritt/rustscan/releases/download/v$pkgver/rustscan-$pkgver-x86_64.tar.gz")
sha256sums=("63f8386a20044843bf3f70cf963971a4c413382c20c0c9d8343e1423b28bec77")

package() {
    install -Dm755 rustscan -t "$pkgdir/usr/bin/"
}
