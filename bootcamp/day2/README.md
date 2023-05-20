# Homework diary 📔

2023年5月に開催された[Motoko Bootcamp Day 2](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD)のプロジェクトをRust言語で実装します。

## 1. Rustプロジェクト作成

Rustのプロジェクト「day2」を作成します。`cargo new`コマンドを`--lib`オプションを付与して実行します。

```
$ cargo new day2 --lib
$ cd day2
```

生成されたファイルは以下の通りです。

```
day2
├── Cargo.toml
└── src
    └── lib.rs
```

## 2. Cargo.tomlの編集

### (1) IC関連ライブラリ追加

[ic-cdk](https://docs.rs/ic-cdk/latest/ic\_cdk/)と[ic-cdk-macros](https://docs.rs/ic-cdk-macros/latest/ic\_cdk\_macros/)ライブラリを使用します。最新バージョンでよいかと思いますので、以下のように実行ます。

```bash
$ cargo add ic-cdk ic-cdk-macros
```

### (2) crate-type設定

Canister上から関数が正しく呼び出させるようcrate-typeを`cdylib`にします。

```toml
[lib]
crate-type = ["cdylib"]
```

[**Cargo.toml**](Cargo.toml)

## 3. dfx.jsonの作成

Canisterの定義を行います。

[**dfx.json**](dfx.json)

```json
{
  "canisters": {
    "day2": {
      "candid": "./day2.did",
      "package": "day2",
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

## 4. candidの作成

dfx.jsonの [canisters] > [day12 > [candid]項目に指定したファイルに、Canisterが提供する関数のI/Fを定義します。

[Motoko Bootcamp Day2 📺 Interface](https://github.com/motoko-bootcamp/motoko-starter/blob/main/days/day-2/project/README.MD#-interfacee)に相当するcandidを用意します。

[**day2.did**](day2.did)

```
★
```

MotokoのTimeはint (System time is represent as nanoseconds since 1970-01-01.)
→Rust的にはint128でOK。

https://internetcomputer.org/docs/current/motoko/main/base/Time
