# Maintainer: Bee <brandon@skerritt.blog>
pkgname=rustscan-bin
pkgver=1.4.1
pkgrel=1
pkgdesc="Faster Nmap Scanning with Rust"
url="https://github.com/rustscan/rustscan"
license=("MIT")
arch=("x86_64")
provides=("rustscan")
options=("strip")
source=("https://github.com/rustscan/rustscan/releases/download/v$pkgver/rustscan-$pkgver-x86_64.tar.gz")
sha256sums=("4fc0966ff722fb2343d95b2a1772e1c497f86c81e446871e40c2a0dc987ce782")

package() {
    install -Dm755 rustscan -t "$pkgdir/usr/bin/"
}
