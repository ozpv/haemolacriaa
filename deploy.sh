#!/bin/sh

# nixos 
patchelf --set-interpreter /usr/lib64/ld-linux-x86-64.so.2 ./target/release/haemolacriaa 

scp ./target/release/haemolacriaa root@haemolacriaa.com:/app/.
scp -r ./target/site root@haemolacriaa.com:/app/.
