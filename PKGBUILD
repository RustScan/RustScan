# Maintainer: Bee <brandon@skerritt.blog>
pkgname=RustScan-bin
pkgver=0.1.0
pkgrel=1
pkgdesc="Faster Nmap Scanning with Rust"
url="https://github.com/brandonskerritt/rustscan"
license=("GNU")
arch=("x86_64")
provides=("RustScan")
options=("strip")
source=("https://github.com/brandonskerritt/rustscan/releases/download/v$pkgver/RustScan-$pkgver-x86_64.tar.gz")
sha256sums=("0ab268ae43a79cad6e9f75b650629208061590b3311780d55b8a03d35a8d90b0")

package() {
    install -Dm755 RustScan -t "$pkgdir/usr/bin/"
}
