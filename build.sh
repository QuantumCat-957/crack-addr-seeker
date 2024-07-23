#!/bin/bash

cargo install cross

# rustup toolchain add nightly-x86_64-unknown-linux-gnu

# 下载所需的Docker镜像：
docker pull ghcr.io/cross-rs/x86_64-unknown-linux-gnu:0.2.5 --platform linux/amd64

# 将下载的镜像加载到本地Docker环境中：
docker image save -o x86_64-unknown-linux-gnu.tar ghcr.io/cross-rs/x86_64-unknown-linux-gnu:0.2.5
docker image load -i x86_64-unknown-linux-gnu.tar

# 确保你对/target目录具有写入权限
chmod -R 777 ./target

# cross工具使用本地的镜像进行构建：
cross build --target x86_64-unknown-linux-gnu --release 