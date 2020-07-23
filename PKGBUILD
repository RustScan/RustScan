# Maintainer: Bee <brandon@skerritt.blog>
pkgname=rustscan-bin
pkgver=1.1.0
pkgrel=1
pkgdesc="Faster Nmap Scanning with Rust"
url="https://github.com/brandonskerritt/rustscan"
license=("MIT")
arch=("x86_64")
provides=("rustscan")
depends=('rustup')

package() {
	rustup toolchain install stable && cargo install rustscan
}
