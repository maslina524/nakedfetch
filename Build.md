# Windows & Linux

```bash
$ cargo build --release
```

## For linux install Lua

```bash
$ sudo pacman -Sy --needed --noconfirm lua54
```

## Or build

```bash
$ sudo pacman -S --needed base-devel readline
$ cd /tmp
$ curl -RLO https://www.lua.org/ftp/lua-5.4.7.tar.gz
$ tar zxf lua-5.4.7.tar.gz
$ cd lua-5.4.7
$ make linux-readline
$ sudo install -Dm644 src/liblua.a /usr/lib/liblua5.4.a
```

# Android Build

## Install toolchains

``` bash
$ rustup toolchain install nightly
$ rustup component add rust-src --toolchain nightly
$ rustup target add aarch64-linux-android --toolchain nightly
```
## Download NDK

```bash
$ cd ~
$ wget https://dl.google.com/android/repository/android-ndk-r27c-linux.zip
$ unzip android-ndk-r27c-linux.zip
$ ~/android-ndk-r27c/ndk-build --version # Check
```

## Set env

### Bash

```bash
$ export ANDROID_NDK_HOME=$HOME/android-ndk-r27c
$ export ANDROID_NDK_ROOT=$ANDROID_NDK_HOME
$ export ANDROID_PLATFORM=24
$ export PATH=$ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin:$PATH
```

### Fish

```bash
$ set -gx ANDROID_NDK_HOME $HOME/android-ndk-r27c
$ set -gx ANDROID_NDK_ROOT $ANDROID_NDK_HOME
$ set -gx ANDROID_PLATFORM 24
$ fish_add_path $ANDROID_NDK_HOME/toolchains/llvm/prebuilt/linux-x86_64/bin
```

## Build

```bash
$ cargo ndk -t arm64-v8a --platform 24 build --release
```