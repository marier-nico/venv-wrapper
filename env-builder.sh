#!/bin/bash

mkdir -p ./venv
mkdir -p ./venv/bin
mkdir -p ./venv/include
mkdir -p ./venv/lib/python3.9/site-packages

ln -s venv/lib venv/lib64

cat >./venv/pyvenv.cfg <<EOF
home = /usr/bin
include-system-site-packages = false
version = 3.9.5
EOF

ln -s /usr/bin/python3.9 venv/bin/python3.9
ln -s $(pwd)/venv/bin/python3.9 venv/bin/python3
ln -s $(pwd)/venv/bin/python3.9 venv/bin/python

venv/bin/python3.9 -Im ensurepip --upgrade --default-pip
