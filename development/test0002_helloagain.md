# Hello, again

[前回](./test0001_hello)はdfx newコマンドを使ってプログラムを生成したけれども、出力されるファイルが多すぎて分かりづらいので、スクラッチからビルドしてみることにしましょう。

本ドキュメントではBackendを中心に解説しますのでFrontendは用意しません。

## 1. プロジェクトの作成

まずはRustの一般的なライブラリプロジェクトを作成します。

「test0002_helloagain」はプロジェクト名ですので、任意に設定して構いません。


```
$ cargo new test0002_helloagain --lib
$ cd test0002_helloagain
```

## 2. プロジェクト資材準備

### (1) dfx.json

```dfx.json
{
  "canisters": {
    "helloagain": {
      "candid": "./helloagain.did",
      "package": "test0002_helloagain",
      "type": "rust"
    }
  },
  "defaults": {
    "build": {
      "args": "",
      "packtool": ""
    }
  },
  "version": 1
}
```

### (2) didファイルの作成

###### helloagain.did

```
service : {
    "greet": (text) -> (text) query;
}
```
### (3) Cargo.toml修正

```
[package]
name = "test0002_helloagain"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
```

#### a. create-type追加

ライブラリセクションを追加するとよいでしょうか。cdylib は最終成果物の .wasm ファイル（＝動的ライブラリ）を

```
[lib]
crate-type = ["cdylib"]
```
#### b. create-type追加

```bash
$ cargo add candid ic-cdk ic-cdk-macros
```
### (4) Cargo.lock作成

一般にcargo buildを実行すればCargo.lockは生成される。一方、後述のdfx buildコマンド (dfx deploy)では、内部的にcargo buildコマンドを「--locked」オプションで実行するため、事前にCargo.lockファイルを用意しておく必要がある。

```bash
$ cargo generate-lockfile
```

### (5) プログラム

##### src/lib.rs

```rust
#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

## 2. ビルド

```
$ dfx start --clean --background
```

## (1) Canister作成

```
$ dfx canister create helloagain
```

## (2) ビルド

```
$ dfx build
```

### (3) 配備

```
$ dfx deploy
```
