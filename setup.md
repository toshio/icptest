# ⚙ 開発環境準備

Internet ComputerのCanister上で動作するDappsを開発するための環境を準備します。

Internet Computerでは、Dappsの開発言語としてMotoko、Rust、JavaScriptなどいくつかありますが、このドキュメントではRust言語を使って開発する前提で進めていきます。

## 1. SDKインストール

まず、dfxコマンドなど開発に必要なツールをインストールします。

https://internetcomputer.org/docs/current/developer-docs/setup/install/


```bash
$ sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"
```

## 2. Rustインストール

このドキュメントでは、Rust言語で開発する想定で説明します。

Rust言語は、[Rust公式サイト](https://www.rust-lang.org/tools/install)を参考にインストールするとよいでしょう。

筆者はWSL (Windows Subsystem for Linux)上のUbuntu 22.04を使用しており、以下のコマンドでインストールできました。

```bash
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### wasm

```bash
$ rustup target add wasm32-unknown-unknown
```

## 3. cmakeインストール

Rust CDKではcmakeが必要になるのでインストールしておきます。

```bash
$ sudo apt install cmake
```

## 4. エディタ

Visual Stuido Codeなどを利用すると便利かもしれません。