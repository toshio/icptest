# データ更新/参照

Canisterにデータを保存するにはどうすればよいのか。

RustもICPも初学者の自分にとって、以下の内容は急に難易度が高くなって消化不良に陥ってしまった。

https://internetcomputer.org/docs/current/developer-docs/backend/rust/rust-profile

そこで、まずはCanister内にただ一つの文字列を設定/参照するサンプルを考えることにした。


###### dfx.json

```
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

```
service : {
    "set": (text) -> ();
    "get": () -> (text) query;
}
```

##### Cargo.toml

```
[package]
name = "test0003_update"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[lib]
crate-type = ["cdylib"]

[dependencies]
candid = "0.8.4"
ic-cdk = "0.7.0"
ic-cdk-macros = "0.6.8"
```

##### src/lib.rs

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
