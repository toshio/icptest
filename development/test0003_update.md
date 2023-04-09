# データ更新/参照

Canisterにデータを保存するにはどうすればよいでしょうか。

RustもICPも初学者の自分にとって、以下の内容は急に難易度が高くなって消化不良に陥ってしまいました。

https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-profile

そこで、まずはユーザ関係なくCanister内にただ一つの文字列を設定/参照するサンプルを考えることにしました。

## ファイル構成

任意のディレクトリに以下の4つのファイルは作成します。

`Cargo.toml`と`src/lib.rs`はRust関連のファイル、`dfx.json`と `canister.did` （didファイル名は `dfx.json`内に記載の と整合性がとれていれば変えても構いません）は ICP Dapp用のファイルです。

- dfx.json
- canister.did
- Cargo.toml
- src/lib.rs

はじめのうちは各ファイルの関連性がよく分かりませんが、何度かDappsサンプルを書いていくうちになんとなく分かってくると思います。

###### dfx.json

```json
{
  "canisters": {
    "update": {
      "candid": "./candid.did",
      "package": "test0003_update",
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

###### canister.did

set()とget()の2つのI/Fを用意することにしました。set()の引数は文字列、get()の戻り値は文字列です。更新を伴わない参照系のget()には `query` を指定すると良いでしょう。

```
service : {
    "set": (text) -> ();
    "get": () -> (text) query;
}
```

##### Cargo.toml

`cargo new <プロジェクト名> --lib`で新規作成したものに、`[lib]`セクションと、`[dependencies]`セクションを追加したものです。

```
[package]
name = "test0003_update"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.8.4"
ic-cdk = "0.7.0"
ic-cdk-macros = "0.6.8"
```

##### src/lib.rs

私自身、Rust初学者でICP Dapps開発も間もないので、試行錯誤しつつ少しずつ理解しながらという手探り状態ですので最適なプログラムとはいえませんが、以下、作成したプログラムを示します。

```rust
use std::cell::RefCell;

thread_local! {
    static VALUE: RefCell<String> = RefCell::default();
}

#[ic_cdk_macros::update]
fn set(text: String) {
    VALUE.with(|value| {
        *value.borrow_mut() = text;
    });
}

#[ic_cdk_macros::query]
fn get() -> String {
    VALUE.with(|value| {
        value.borrow().clone()
    })
}
```

### 解説

#### (1) set()関数

サンプルプログラム内で定義している`set()`関数は、更新系処理となるため、Outer attributeに`ic_cdk_macros::update`を指定します。

#### (2) get()関数

サンプルプログラム内で定義した`set()`関数は、get()関数は、更新系処理となるため、Outer attributeに`ic_cdk_macros::query`を指定します。

#### (3) thread_local

Canisterは、

[Intro to Building on the IC in Rust]

https://youtu.be/163yRgrOSC8?t=461