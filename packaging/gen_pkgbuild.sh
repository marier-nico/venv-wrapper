#!/bin/bash

read -d '' PKGBUILD << EOF
# Maintainer: Nicolas Marier <aur at nmarier dot com>
pkgname=venv-wrapper-bin
pkgver=$PKGVER
pkgrel=1
pkgdesc='Simple python virtual environment management'
url='https://github.com/marier-nico/venv-wrapper'
source=("\$pkgname-\$pkgver::https://github.com/marier-nico/venv-wrapper/releases/download/v\$pkgver/venv-wrapper")
noextract=("\$pkgname-\$pkgver")
arch=('x86_64')
license=('Apache')
depends=()
optdepends=()
conflicts=('venv-wrapper-bin')
provides=('venv-wrapper-bin')
sha256sums=('$SHA256SUM')

package() {
  cd "\$srcdir/"

  install -Dm755 \$pkgname-\$pkgver "\${pkgdir}/usr/bin/venv-wrapper"
}
EOF

echo "$PKGBUILD"
