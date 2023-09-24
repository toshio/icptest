# テンプレート生成 (dfx new)

まずはじめに、最もシンプルなHelloを返すDappsを開発してみます。

## プロジェクト作成

以下のように`dfx new`コマンドを使うことで、Rust用のサンプルアプリケーションを自動生成することができます。

『icptest』はプロジェクト名ですので、適宜変更しても構いません。

```bash
$ dfx new --type=rust icptest
$ cd icptest/
```

##### 補足

以下の記事より、`dfx new`コマンドは、v0.15.1で大幅なアップデートがある予定で、Motoko、Rust以外に、Azle (JavaScript)やKybra (Python)も利用できるようになるとのことです。また、FrontendのテンプレートもReact、Vue、Unit Testの有無、Internet Identityなども選べるようになるようですので、リリースされ次第記事を更新します。

https://internetcomputer.org/blog/2023/09/01/news-and-updates/update

>The dfx new command has some exciting updates and improvements that will be coming out in dfx release 0.15.1. One of the biggest changes is that there will be an interactive prompt that will allow you to choose what language template you'd like to create a new project using. Previously, this decision was made using the flag --type=motoko or --type=rust. Additionally, there will be additional options through this new interactive prompt, such as Azle and Kybra.

>In this interactive prompt, there are also options to enable add-ons, such as adding a frontend canister using different frontend templates like React or Vue, enabling unit tests, or adding integrations like Internet Identity and Bitcoin.


## ローカル環境でのCanister起動

Dapps開発時は本番環境に配備する必要はなく、ローカルPC内に配備します。

`dfx start`コマンドで、ローカルPC環境で動作するCanisterを起動します。サービスを起動した後にコマンドラインを復帰させたい場合「--background」オプションを付与するとよいでしょう。

「--clean」はサービス起動時にCanisterを初期化するためのオプションです。

```bash
$ dfx start --clean --background
Running dfx start for version 0.13.1
Using the default definition for the 'local' shared network because /home/toshio/.config/dfx/networks.json does not exist.
Dashboard: http://localhost:38281/_/dashboard
```

## ローカル環境へのDeploy

```bash
$ dfx deploy
︙
Deployed canisters.
URLs:
  Frontend canister via browser
    icptest_frontend: http://127.0.0.1:8000/?canisterId=ryjl3-tyaaa-aaaaa-aaaba-cai
  Backend canister via Candid interface:
    icptest_backend: http://127.0.0.1:8000/?canisterId=r7inp-6aaaa-aaaaa-aaabq-cai&id=rrkah-fqaaa-aaaaa-aaaaq-cai
```

## 実行

Webブラウザーでそれぞれアクセスしてみましょう。

### Dashboard

![](.gitbook/assets/template/01_dashboard.png)

### Frontend

![](.gitbook/assets/template/02_frontend.png)

### Backend

![](.gitbook/assets/template/03_backend.png)

## 解説

生成されたアプリケーションは大きくFrontendとBackendの2種類があります。本ドキュメントではとくにBackend側に着目して、どのような仕組みとなっているかを紐解き、1ステップずつ開発方法を学んでいきたいと思います。

生成されたファイルのうち、Backendを動作させるのに最低限必要な設定ファイルは以下となります。
- dfx.json
- didファイル
- Cargo.toml
- lib.rs

`dfx new`コマンドには、『--no-frontend』オプションがあってBackendのみ生成もできそうなのですが、執筆時点で最新のV0.13.1ではFrontendも同時に出力されてしまうようです。

###### dfx.json

```json
{
  "canisters": {
    "icptest_backend": {
      "candid": "src/icptest_backend/icptest_backend.did",
      "package": "icptest_backend",
      "type": "rust"
    },
    "icptest_frontend": {
      "dependencies": [
        "icptest_backend"
      ],
      "frontend": {
        "entrypoint": "src/icptest_frontend/src/index.html"
      },
      "source": [
        "src/icptest_frontend/assets",
        "dist/icptest_frontend/"
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
  "output_env_file": ".env",
  "version": 1
}
```

###### src/icptest_backend/icptest_backend.did

開発したアプリケーションは外部からどのように呼ばれるか、I/Fを規定します。

```text
service : {
    "greet": (text) -> (text) query;
}
```

##### src/icptest_backend/Cargo.toml

```ini
[package]
name = "icptest_backend"
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

###### src/icptest_backend/src/lib.rs

```rust
#[ic_cdk_macros::query]
fn greet(name: String) -> String {
    format!("Hello, {}!", name)
}
```
