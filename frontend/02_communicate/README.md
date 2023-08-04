# キャニスターとの通信

FrontendからCanisterへ通信を行うサンプルは、『[テンプレート生成](../../development/template)』でも解説した`dfx new`コマンドを使用して出力されたコードを必要に応じて修正していくことから慣れていくのがよいでしょう。

使用するFrame Workに合わせたBoilerplateは、ICPの様々な開発者がGitHubでも公開されていますから、検索してみてください。

## 1. プロジェクトの作成

```bash
$ dfx new --type=rust icptest
```

テンプレートが生成されます。

```
icptest
├── .env
├── .gitignore
├── Cargo.lock
├── Cargo.toml
├── README.md
├── dfx.json
├── README.md
├── node_modules
│   ︙
├── package-lock.json
├── package.json
├── src
│   ├── icptest_backend
│   │   ├── Cargo.toml
│   │   ├── icptest_backend.did
│   │   └── src
│   │       └── lib.rs
│   └── icptest_frontend
│       ├── assets
│       │   ├── .ic-assets.json5
│       │   ├── favicon.ico
│       │   ├── logo2.svg
│       │   ├── main.css
│       │   └── sample-asset.txt
│       └── src
│           ├── .ic-assets.json5
│           ├── index.html
│           └── index.js
└── webpack.config.js
```

## 2. Backend作成 (変更)

テンプレート出力されたら、まずはBackend側を用意します。

ここでは、didファイルでI/Fを定義、Tomlの依存パッケージを最新化、Rustのソースを修正という流れで、[2. データ更新/参照](../../backend/02_update/)で解説したシンプルなBackendとします。

### (1) didファイル作成（修正）

##### [src/icptest_backend/icptest_backend.did](src/icptest_backend/icptest_backend.did)

```
service : {
    "set": (text) -> ();
    "get": () -> (text) query;
}
```

### (2) 依存パッケージ変更

##### [src/icptest_backend/Cargo.toml](src/icptest_backend/Cargo.toml)

出力されるテンプレートのライブラリバージョンが古い場合には最新化しておきましょう。

Rustでは、[[`cargo install cargo-edit`:https://docs.rs/crate/cargo-edit/latest]]というコマンドをインストールして、`cargo upgrade`コマンドで依存パッケージを最新化できるようですが、自分の環境ではうまく行かなかったので、`cargo rm` & `cargo add`コマンドで最新化する手順を行います。今回のサンプルでは使わないライブラリも削除しておきます。

```bash
$ cd src/icptest_backend
$ cargo rm candid ic-cdk ic-cdk-timers
$ cargo add candid ic-cdk
```

###### 変更前 (2023/08/13時点)

```toml
︙
[dependencies]
candid = "0.8"
ic-cdk = "0.7"
ic-cdk-timers = "0.1" # Feel free to remove this dependency if you don't need timers
```

###### 変更後 (2023/08/13時点)

```toml
︙
[dependencies]
candid = "0.9.3"
ic-cdk = "0.10.0"
```

### (3) Rustソース修正

##### [src/icptest_backend/src/lib.rs](src/icptest_backend/src/lib.rs)

```rust
use std::cell::RefCell;

thread_local! {
    static VALUE: RefCell<String> = RefCell::default();
}

#[ic_cdk::update]
fn set(text: String) {
    VALUE.with(|value| {
        *value.borrow_mut() = text;
    });
}

#[ic_cdk::query]
fn get() -> String {
    VALUE.with(|value| {
        value.borrow().clone()
    })
}
```

### (4) Backendビルド

カレントディレクトリ（src/icptest_backend/）から、プロジェクトディレクトリへ移動して、ビルドを行います。

Local Network上でCanisterを起動した状態にして、Backendをビルド・配備します。

```bash
$ cd ../../
$ dfx start --background --clean
$ dfx deploy
```

`dfx deploy`コマンドを実行するとBackendに続いてFrontendのビルドも行われます。Frontend側はまだ修正していないため、本手順の後に再度ビルドが必要ですが、didファイルからFrontend向けのJavaScriptソースを生成するために実行します。以下の方法でBackendのみビルドするのでも構いません。

```bash
$ dfx deploy icp_backend
$ dfx generate icptest_backend
Generating type declarations for canister icptest_backend:
  src/declarations/icptest_backend/icptest_backend.did.d.ts
  src/declarations/icptest_backend/icptest_backend.did.js
  src/icptest_backend/icptest_backend.did
```

## 3. Frontend作成 (変更)

frontendは、src/icptest_frontend/ディレクトリ内に、画像などの資材を格納するassetsディレクトリと、HTMLやJavaScriptなどを格納するsrcディレクトリが作成されています。

今回用意したBackendに合わせて、以下の2ファイルを編集する。

- index.html
- index.js

##### [src/icptest_frontend/src/index.html](src/icptest_frontend/src/index.html)

```html
<!DOCTYPE html>
<html lang="en">
  <head>
    <meta charset="UTF-8" />
    <meta name="viewport" content="width=device-width" />
    <title>icptest</title>
  </head>
  <body>
    <main>
      <br />
      <input id="text" type="text" />
      <button id="buttonSet">Set</button>
      <br />
      <button id="buttonGet">Get</button>
      <input id="result" type="text" disabled />
    </main>
  </body>
</html>
```

##### [src/icptest_frontend/src/index.html](src/icptest_frontend/src/index.html)

```javascript
import { icptest_backend } from "../../declarations/icptest_backend";

document.getElementById("buttonSet").addEventListener("click", async (e) => {
  try {
    const value = document.getElementById("text").value.toString();
    e.target.setAttribute("disabled", true);
    await icptest_backend.set(value);
  } catch (e) {
    alert(e);
  } finally {
    e.target.removeAttribute("disabled");
  }
  return true;
});

document.getElementById("buttonGet").addEventListener("click", async (e) => {
  try {
    e.target.setAttribute("disabled", true);
    const value = await icptest_backend.get();
    document.getElementById("result").value = value;
  } catch (e) {
    alert(e);
  } finally {
    e.target.removeAttribute("disabled");
  }
  return true;
});
```

### 補足説明

クライアント側では、`dfx generate`コマンドで生成されたソースをimportして、didで定義した関数を呼び出すだけで、Canisterとのやりとりが簡単に行うことができる。通信処理等の泥臭い部分は隠蔽されています。

```javascript
import { icptest_backend } from "../../declarations/icptest_backend";
```

```javascript
await icptest_backend.set(value);
```

```javascript
const value = await icptest_backend.get();
```

## 4. deploy

Frontendの作成が終わったらCanisterへdeployします。

```bash
$ dfx deploy
```

`dfx deploy`コマンドの実行により、修正したhtmlやJavascriptなどのリソースがWepackでまとめられ、dist/icptest_frontendディレクトリに出力されます。

配備先がローカル開発環境かICネットワークかといった違いのほか、環境により割り当てられたCanister Idも変わってきます。

`dfx new`コマンドで生成されたテンプレートでは、そうしたwebpack.config.js ファイルや .envファイルで吸収しているようです。（[dfx.json](dfx.json)のoutput_env_file項目を指定することで.envに書き出されるようです）

この辺りは利用するWeb Frameworkや開発者によっても違いがあり、まだ色々と試行錯誤が繰り返されている状況ですので、将来的にやり方も変わるかもしれません。
