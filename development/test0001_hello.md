# Hello

まずはじめに、最もシンプルなHelloを返すDappsを開発してみます。

## プロジェクト作成

以下のようにdfx newコマンドを使うことで、Rust用のサンプルアプリケーションを自動生成することができます。

『test0001_hello』はプロジェクト名ですので、適宜変更しても構いません。

```bash
$ dfx new --type=rust test0001_hello
$ cd test0001_hello/
```

## ローカル環境でのCanister起動

Dapps開発時は本番環境に配備する必要はなく、ローカルPC内に配備します。

dfx startコマンドで、ローカルPC環境で動作するCanisterを起動します。サービスを起動した後にコマンドラインを復帰させたい場合「--background」オプションを付与するとよいでしょう。

「--clean」はサービス起動時にCanisterを初期化するためのオプションです。

```bash
$ dfx start --clean --background
Running dfx start for version 0.12.1
Using the default definition for the 'local' shared network because /home/toshio/.config/dfx/networks.json does not exist.
Dashboard: http://localhost:43839/_/dashboard
```

## ローカル環境へのDeploy

```bash
$ dfx deploy
︙
Deployed canisters.
URLs:
  Frontend canister via browser
    test0001_hello_frontend: http://127.0.0.1:4943/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai
  Backend canister via Candid interface:
    test0001_hello_backend: http://127.0.0.1:4943/?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
```

## 実行

Webブラウザーでそれぞれアクセスしてみましょう。

### Dashboard

![](../.gitbook/assets/test0001_hello_01_dashboard.png)

### Frontend

![](../.gitbook/assets/test0001_hello_02_frontend.png)

### Backend

![](../.gitbook/assets/test0001_hello_03_backend.png)

## 解説

生成されたアプリケーションは大きくFrontendとBackendの2種類があります。本ドキュメントでは、主にBackend側に着目して、どのような仕組みとなっているかを紐解き、1ステップずつ開発方法を学んでいきたいと思います。

Backendを動作させるのに最低限必要な設定は

- dfx.json
- didファイル
- Cargo.toml
- lib.rs

なお、dfx newコマンドには、『--no-frontend』オプションがあってBackendのみ生成もできそうなのですが、執筆時点で最新のV0.12.1ではFrontendも同時に出力されてしまうようです。

###### dfx.json

```json
{
  "canisters": {
    "test0001_hello_backend": {
      "candid": "src/test0001_hello_backend/test0001_hello_backend.did",
      "package": "test0001_hello_backend",
      "type": "rust"
    },
    "test0001_hello_frontend": {
      "dependencies": [
        "test0001_hello_backend"
      ],
      "frontend": {
        "entrypoint": "src/test0001_hello_frontend/src/index.html"
      },
      "source": [
        "src/test0001_hello_frontend/assets",
        "dist/test0001_hello_frontend/"
      ],
      "type": "assets"
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

###### src/test0001_hello_backend/test0001_hello_backend.did

開発したアプリケーションは外部からどのように呼ばれるか、I/Fを規定します。

```text
service : {
    "greet": (text) -> (text) query;
}
```

##### src/test0001_hello_backend/Cargo.toml

```
[package]
name = "test0001_hello_backend"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.8.2"
ic-cdk = "0.6.0"
ic-cdk-macros = "0.6.0"
```

###### src/test0001_hello_backend/src/lib.rs

```rust
#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```
