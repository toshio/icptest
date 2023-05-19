# 1. Hello

[テンプレート生成](../template.md)では`dfx new`コマンドを使ってプログラムを生成しましたが、FrontendとBackendの両方のソースが出力され、ファイルが多すぎて分かりづらいかと思います。

そのため、テンプレート生成を使用せずに、スクラッチからビルドしてみることにしましょう。

本ドキュメントではBackendを中心に解説しますので、ここではFrontendは用意しません。

以下のような構成で作成しますので、実際のプロジェクトでは変更してください。

|項目|値|
|:----|:-----|
|Project Name|icptest|
|Canister Name|backend|

## 1. プロジェクトの作成

まずはRustの一般的なライブラリプロジェクトを作成します。

「icptest」はプロジェクト名ですので、任意に設定して構いません。

```bash
$ cargo new icptest --lib
$ cd icptest
```

## 2. プロジェクト資材準備

### (1) dfx.jsonの作成

以下のようなdfx.jsonを用意してみましょう。

###### [dfx.json](dfx.json)

```json
{
  "canisters": {
    "backend": {
      "candid": "./backend.did",
      "package": "icptest",
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

###### [backend.did](backend.did)

```
service : {
    "greet": (text) -> (text) query;
}
```
### (3) [Cargo.toml](Cargo.toml)編集

```ini
[package]
name = "icptest"
version = "0.1.0"
edition = "2021"

[dependencies]
```

#### a. crate-type追加

ライブラリセクションを追加して、crate-typeにcdylibを指定します。cdylib を指定することで最終成果物の .wasm ファイルを動的ライブラリにします。

```ini
[lib]
crate-type = ["cdylib"]
```
#### b. dependencies追加

```bash
$ cargo add candid ic-cdk ic-cdk-macros
```

### (4) Cargo.lock作成

一般にcargo buildを実行すればCargo.lockは生成されます。しかしながら、後述のdfx buildコマンド (dfx deploy)では、内部的にcargo buildコマンドを「--locked」オプションで実行しているようですので、事前にCargo.lockファイルを用意しておく必要があります。

```bash
$ cargo generate-lockfile
```

### (5) プログラム

##### [src/lib.rs](src/lib.rs)

```rust
#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```

## 2. ビルド

DappをビルドしてローカルPC上にCanisterに配置する手順を以下に示します。

### (1) サービス起動

まずは、ローカルPC上にCanister実行環境を起動します。`dfx start`コマンドで行います。以下に実行例を示します。

```bash
$ dfx start --clean --background
```

起動オプションの詳細は[公式ドキュメント](https://internetcomputer.org/docs/current/references/cli-reference/dfx-start)を参考にするとよいでしょう。

`--clean`オプションをつけると、起動時にCanisterを初期化します。初回起動、もしくは`--clean`でサービスを起動した時点ではCanisterは一つも登録されていません。

`--background`オプションをつけるとコマンドが復帰し、サービスがバックグラウンドで起動されます。起動したサービスは、`dfx stop`コマンドで停止できます。

### (2) Canister作成

作成したDappを配備先のCanisterを作成します。

```bash
$ dfx canister create backend
```

Canisterが一つも登録されていない状況から本コマンドを実行した場合、作成するCanisterのほかに、Wallet Canisterが作成されます。Internet ComputerではDappを実行させるためのCycleと呼ばれる燃料に相当するものが必要です。

本番環境では ICPトークンをCycleに変換してDappを実行します。ローカル環境の場合には、コストはかかりませんが Cycle の仕組みがあり、作成されたWallet Canisterで管理されています。
### (3) ビルド

Canisterに配備するwasmモジュールをビルドします。

```bash
$ dfx build
```

`target/wasm32-unknown-unknown/release/`ディレクトリにwasmモジュールが生成され、Canister向けに最適化されたモジュールが、`./.dfx/local/canisters/<Canister名>/<canister名>.wasm`に格納されます。

### (4) 配備

```bash
$ dfx deploy
```

簡易UIのCanisterも用意されます。このCanisterを使うことで、WebページからDappへ簡易のリクエストを発行できます。

### 補足

なお、手順(2)～(3)は、`dfx deploy`コマンドを実行する過程で必要に応じて実行されます。説明のため記載しましたが、実際の作業では省略して構いません。

## まとめ

各ファイルと定数の関係を以下に示します。

![](../../.gitbook/assets/backend/01_hello/01_relation.drawio.png)
