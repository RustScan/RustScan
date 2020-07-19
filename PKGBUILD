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
sha256sums=("b29e62903d1577fafee819c06ca2a6660da8a45ab74d46e4a14b6ce95a6892ab")

package() {
    install -Dm755 rustscan -t "$pkgdir/usr/bin/"
}
